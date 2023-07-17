use crate::{Color, Goban, Point};
use std::marker::PhantomData;

#[derive(Debug, PartialEq, Eq)]
pub enum SearchStep {
    Reject(Point),
    Match(Color, Point),
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
    type Item = (Color, Point);

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            match self.searcher.next() {
                SearchStep::Done => { return None; },
                SearchStep::Match(color, at) => { return Some((color, at)); },
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
                0xffff => SearchStep::Match(Color::Black, (x, y).into()),

                //   x x
                // x   x
                // x x x
                0x3fff | 0xf3ff | 0xffcf | 0xfffc => SearchStep::Match(Color::Black, (x, y).into()),

                // o x x
                // x   x
                // x x x
                0xbfff | 0xfbff | 0xfffe | 0xffef => SearchStep::Match(Color::Black, (x, y).into()),

                // - - -
                // x   x
                // x x x
                0xf77d | 0xffd5 | 0x7ddf | 0x57ff => SearchStep::Match(Color::Black, (x, y).into()),

                // - - -
                // -   x
                // - x x
                0xf755 | 0x7dd5 | 0x55df | 0x577d => SearchStep::Match(Color::Black, (x, y).into()),

                // o o o
                // o   o
                // o o o
                0xaaaa => SearchStep::Match(Color::White, (x, y).into()),

                //   o o
                // o   o
                // o o o
                0x2aaa | 0xa2aa | 0xaa8a | 0xaaa8 => SearchStep::Match(Color::White, (x, y).into()),

                // x o o
                // o   o
                // o o o
                0xeaaa | 0xaeaa | 0xaaa9 | 0xa6aa => SearchStep::Match(Color::White, (x, y).into()),

                // - - -
                // o   o
                // o o o
                0x699a | 0xaa96 | 0xa669 | 0x56aa => SearchStep::Match(Color::White, (x, y).into()),

                // - - -
                // -   o
                // - o o
                0x559a | 0x6995 | 0xa655 | 0x5669 => SearchStep::Match(Color::White, (x, y).into()),

                _ => SearchStep::Reject((x, y).into())
            }
        }
    }
}

impl<'a> IntoIterator for EyeSearcher<'a> {
    type Item = (Color, Point);
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
                Some(Color::Black) => 3,
                Some(Color::White) => 2,
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

pub struct Eye;

impl Eye {
    pub fn new() -> Self {
        Self { }
    }
}

impl<'a> Pattern<'a> for Eye {
    type Searcher = EyeSearcher<'a>;

    fn into_searcher(self, goban: &'a Goban) -> Self::Searcher {
        EyeSearcher {
            goban,
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
    fn eye_detects_black_eyes() {
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
            Eye::new().into_searcher(&goban).into_iter().collect::<Vec<_>>(),
            vec! [
                (Color::Black, (8u8, 0u8).into()),
                (Color::Black, (1u8, 1u8).into()),
                (Color::Black, (3u8, 2u8).into()),
                (Color::Black, (0u8, 3u8).into()),
            ]
        );
    }

    /// ```
    /// x x x       x
    /// x   x x     x x
    /// x x x   x
    ///   x x x x
    /// x x
    /// ```
    #[test]
    fn eye_detects_white_eyes() {
        let mut goban = Goban::new(9, 9);
        goban.play((0u8, 0u8).into(), Color::White);
        goban.play((0u8, 1u8).into(), Color::White);
        goban.play((0u8, 2u8).into(), Color::White);
        goban.play((0u8, 4u8).into(), Color::White);
        goban.play((1u8, 0u8).into(), Color::White);
        goban.play((1u8, 2u8).into(), Color::White);
        goban.play((1u8, 3u8).into(), Color::White);
        goban.play((1u8, 4u8).into(), Color::White);
        goban.play((2u8, 0u8).into(), Color::White);
        goban.play((2u8, 1u8).into(), Color::White);
        goban.play((2u8, 2u8).into(), Color::White);
        goban.play((2u8, 3u8).into(), Color::White);
        goban.play((3u8, 1u8).into(), Color::White);
        goban.play((3u8, 3u8).into(), Color::White);
        goban.play((4u8, 2u8).into(), Color::White);
        goban.play((4u8, 3u8).into(), Color::White);
        goban.play((7u8, 0u8).into(), Color::White);
        goban.play((7u8, 1u8).into(), Color::White);
        goban.play((8u8, 1u8).into(), Color::White);

        assert_eq!(
            Eye::new().into_searcher(&goban).into_iter().collect::<Vec<_>>(),
            vec! [
                (Color::White, (8u8, 0u8).into()),
                (Color::White, (1u8, 1u8).into()),
                (Color::White, (3u8, 2u8).into()),
                (Color::White, (0u8, 3u8).into()),
            ]
        );
    }
}