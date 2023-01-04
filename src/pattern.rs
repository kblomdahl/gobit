use crate::{Color, Goban, Point};
use std::marker::PhantomData;

#[derive(Debug, PartialEq, Eq)]
pub enum SearchStep {
    Reject(Point),
    Match(Point),
    Done
}

pub trait Searcher<'a> {
    fn next(&mut self) -> SearchStep;
}

pub trait Pattern<'a> {
    type Searcher: Searcher<'a>;

    fn into_searcher(self, goban: &'a Goban) -> Self::Searcher;
}

pub struct SearcherIter<'a, S: Searcher<'a>> {
    searcher: S,
    phantom: PhantomData<&'a ()>,
}

impl<'a, S: Searcher<'a>> Iterator for SearcherIter<'a, S> {
    type Item = Point;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            match self.searcher.next() {
                SearchStep::Done => { return None; },
                SearchStep::Match(at) => { return Some(at); },
                _ => {},
            }
        }
    }
}

/// Traverse the board from the left to the right, and bottom to top checking
/// the `3x3` pattern around each vertex whether it is an eye. To speed-up the
/// performance of the pattern checking we store the pattern around each point
/// as a bit vector in the following order:
///
/// 8 5 3
/// 7   2
/// 6 4 1
///
/// This allows us to incrementally update the pattern using just a few bit
/// operations when stepping forward.
pub struct EyeSearcher<'a> {
    goban: &'a Goban,
    color: Color,
    finger: (u8, u8),
    pattern: u16,
}

impl<'a> Searcher<'a> for EyeSearcher<'a> {
    fn next(&mut self) -> SearchStep {
        let (x, y) = self.finger;
        let (width, height) = (self.goban.width() as u8, self.goban.height() as u8);

        if y >= height {
            SearchStep::Done
        } else {
            if x == 0 {
                self.pattern = self.pattern_at(x, y);
            } else {
                // ```
                // 8 5 3      5 3 x
                // 7   2  --> x   x
                // 6 4 1      4 1 x
                // ```
                self.pattern = ((self.pattern & 0x303) << 6)
                    | ((self.pattern & 0xf0) << 4)
                    | self.bit_at(x-1, y) << 12
                    | self.bit_at(x+1, y.wrapping_sub(1)) << 4
                    | self.bit_at(x+1, y+0) << 2
                    | self.bit_at(x+1, y+1) << 0;
            }

            self.finger = if x + 1 >= width {
                (0, y + 1)
            } else {
                (x + 1, y)
            };

            match self.pattern {
                // x x x
                // x   x
                // x x x
                0xffff => SearchStep::Match((x, y).into()),

                //   x x
                // x   x
                // x x x
                0x3fff | 0xf3ff | 0xffcf | 0xfffc => SearchStep::Match((x, y).into()),

                // o x x
                // x   x
                // x x x
                0xbfff | 0xfbff | 0xfffe | 0xffef => SearchStep::Match((x, y).into()),

                // - - -
                // x   x
                // x x x
                0xf77d | 0xffd5 | 0x7ddf | 0x57ff => SearchStep::Match((x, y).into()),

                // - - -
                // -   x
                // - x x
                0xf755 | 0x7dd5 | 0x55df | 0x577d => SearchStep::Match((x, y).into()),

                _ => SearchStep::Reject((x, y).into())
            }
        }
    }
}

impl<'a> IntoIterator for EyeSearcher<'a> {
    type Item = Point;
    type IntoIter = SearcherIter<'a, EyeSearcher<'a>>;

    fn into_iter(self) -> Self::IntoIter {
        SearcherIter {
            searcher: self,
            phantom: PhantomData::default(),
        }
    }
}

impl<'a> EyeSearcher<'a> {
    #[inline]
    fn bit_at(&self, x: u8, y: u8) -> u16 {
        if x >= self.goban.width() as u8 || y >= self.goban.height() as u8 {
            1
        } else {
            match self.goban.at((x, y).into()) {
                None => 0,
                Some(color) if color == self.color => 3,
                Some(_) => 2,
            }
        }
    }

    #[inline]
    fn pattern_at(&self, x: u8, y: u8) -> u16 {
        (self.bit_at(x.wrapping_sub(1), y.wrapping_sub(1)) << 14)
        | (self.bit_at(x.wrapping_sub(1), y+0) << 12)
        | (self.bit_at(x.wrapping_sub(1), y+1) << 10)
        | (self.bit_at(x, y.wrapping_sub(1)) << 8)
        | (self.bit_at(x, y+1) << 6)
        | (self.bit_at(x+1, y.wrapping_sub(1)) << 4)
        | (self.bit_at(x+1, y+0) << 2)
        | (self.bit_at(x+1, y+1) << 0)
    }
}

pub struct Eye {
    color: Color,
}

impl Eye {
    pub fn new(color: Color) -> Self {
        Self { color }
    }
}

impl<'a> Pattern<'a> for Eye {
    type Searcher = EyeSearcher<'a>;

    fn into_searcher(self, goban: &'a Goban) -> Self::Searcher {
        EyeSearcher {
            goban,
            color: self.color,
            finger: (0, 0),
            pattern: 0,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// ```
    /// x x x       x
    /// x   x x     x x
    /// x x x   x
    ///   x x x x
    /// x x
    /// ```
    #[test]
    fn eye_detects_eyes() {
        let mut goban = Goban::new(9, 9);
        goban.play((0u8, 0u8).into(), Color::Black);
        goban.play((0u8, 1u8).into(), Color::Black);
        goban.play((0u8, 2u8).into(), Color::Black);
        goban.play((0u8, 4u8).into(), Color::Black);
        goban.play((1u8, 0u8).into(), Color::Black);
        goban.play((1u8, 2u8).into(), Color::Black);
        goban.play((1u8, 3u8).into(), Color::Black);
        goban.play((1u8, 4u8).into(), Color::Black);
        goban.play((2u8, 0u8).into(), Color::Black);
        goban.play((2u8, 1u8).into(), Color::Black);
        goban.play((2u8, 2u8).into(), Color::Black);
        goban.play((2u8, 3u8).into(), Color::Black);
        goban.play((3u8, 1u8).into(), Color::Black);
        goban.play((3u8, 3u8).into(), Color::Black);
        goban.play((4u8, 2u8).into(), Color::Black);
        goban.play((4u8, 3u8).into(), Color::Black);
        goban.play((7u8, 0u8).into(), Color::Black);
        goban.play((7u8, 1u8).into(), Color::Black);
        goban.play((8u8, 1u8).into(), Color::Black);

        assert_eq!(
            Eye::new(Color::Black).into_searcher(&goban).into_iter().collect::<Vec<_>>(),
            vec! [
                (8u8, 0u8).into(),
                (1u8, 1u8).into(),
                (3u8, 2u8).into(),
                (0u8, 3u8).into(),
            ]
        );
    }
}