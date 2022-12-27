use crate::{Color, Point, array2d::Array2D, vertex::Vertex};
use std::{ops::{Index, IndexMut}, iter, collections::HashSet};

pub struct Goban {
    buf: Array2D<Vertex>,
    mark: usize,
}

impl Index<Point> for Goban {
    type Output = Vertex;

    fn index(&self, index: Point) -> &Self::Output {
        &self.buf[(index.x(), index.y())]
    }
}

impl IndexMut<Point> for Goban {
    fn index_mut(&mut self, index: Point) -> &mut Self::Output {
        &mut self.buf[(index.x(), index.y())]
    }
}

impl Goban {
    pub fn new(width: usize, height: usize) -> Self {
        let mut goban = Self {
            buf: Array2D::new(width + 2, height + 2, Vertex::invalid()),
            mark: 0,
        };

        for point in goban.iter() {
            goban[point] = Vertex::empty(point);
        }

        goban
    }

    pub fn width(&self) -> usize {
        self.buf.width() - 2
    }

    pub fn height(&self) -> usize {
        self.buf.height() - 2
    }

    pub fn iter(&self) -> impl Iterator<Item=Point> {
        let (mut x, mut y) = (0, 1);
        let width = self.width() - 1;
        let height = self.height();

        iter::from_fn(move || {
            if x > width {
                x = 1;
                y += 1;

                if y > height {
                    return None;
                }
            } else {
                x += 1;
            }

            Some(Point::new(x as u8, y as u8))
        })
    }

    pub fn num_liberties(&self, at: Point) -> usize {
        let mut curr = at;
        let mut liberties = HashSet::new();

        loop {
            for other in curr.neighbours() {
                if self[other].is_valid() && self[other].is_empty() {
                    liberties.insert(other);
                }
            }

            curr = self[curr].next_link();
            if curr == at {
                break
            }
        }

        liberties.len()
    }

    pub fn is_valid(&self, at: Point, color: Color) -> bool {
        let opposite = color.opposite();

        self[at].is_valid() && self[at].is_empty() && at.neighbours()
            .any(|other| {
                self[other].is_valid()
                    && (
                        self[other].is_empty()
                        || (self[other].color() == Some(color) && self.num_liberties(other) > 1)
                        || (self[other].color() == Some(opposite) && self.num_liberties(other) == 1)
                    )
            })
    }

    fn capture_at(&mut self, at: Point) {
        let mut curr = at;
        let mark = self.mark;

        loop {
            self[curr].set_mark(mark);
            self[curr].set_color(None);

            curr = self[curr].next_link();
            if curr == at {
                break
            }
        }
    }

    fn connect_with(&mut self, at: Point, to: Point) {
        let a = self[at].head();
        let b = self[to].head();
        let mark = self.mark;

        if self[a].mark() == self[b].mark() {
            return
        }

        self[a].set_mark(mark);
        self[b].set_mark(mark);

        // given the following two cyclic lists:
        //
        // 1 -> 2 -> .. -> 1 (cyclic)
        // a -> b -> .. -> a (cyclic)
        //
        // becomes
        //
        // 1 -> b -> .. a -> 2 -> .. -> 1
        //
        let a_next = self[a].next_link();
        let b_next = self[b].next_link();
        self[a].set_next_link(b_next);
        self[b].set_next_link(a_next);
    }

    pub fn play(&mut self, at: Point, color: Color) {
        let opposite = color.opposite();
        let mark = self.mark + 1;

        self.mark += 1;
        self[at].set_color(Some(color));
        self[at].set_head(at);
        self[at].set_mark(mark);
        self[at].set_next_link(at);

        for other in at.neighbours() {
            if self[other].color() == Some(opposite) && self.num_liberties(other) == 1 {
                self.capture_at(other);
            } else if self[other].color() == Some(color) {
                self.connect_with(at, other);
            }
        }
    }

    pub fn undo(&mut self) {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn iter_returns_points() {
        assert_eq!(Goban::new(9, 9).iter().count(), 81);
        assert_eq!(Goban::new(13, 13).iter().count(), 169);
        assert_eq!(Goban::new(19, 19).iter().count(), 361);
    }

    #[test]
    fn play_fills_vertices() {
        let mut goban = Goban::new(9, 9);
        goban.play(Point::new(1, 1), Color::Black);
        goban.play(Point::new(2, 1), Color::Black);

        assert_eq!(goban.iter().filter(|at| goban[*at].color() == Some(Color::Black)).count(), 2);
    }

    #[test]
    fn play_clears_captured_vertices() {
        let mut goban = Goban::new(9, 9);
        goban.play(Point::new(1, 1), Color::White);
        goban.play(Point::new(1, 2), Color::Black);
        goban.play(Point::new(2, 1), Color::Black);

        assert_eq!(goban.iter().filter(|at| goban[*at].color() == Some(Color::Black)).count(), 2);
        assert_eq!(goban.iter().filter(|at| goban[*at].color() == Some(Color::White)).count(), 0);
    }

    #[test]
    fn capture_at_clears_all_vertices() {
        let mut goban = Goban::new(9, 9);
        goban.play(Point::new(1, 1), Color::Black);
        goban.play(Point::new(2, 1), Color::Black);
        goban.capture_at(Point::new(1, 1));

        assert_eq!(goban.iter().filter(|at| goban[*at].color() == Some(Color::Black)).count(), 0);
    }

    #[test]
    fn num_liberties_returns_liberties() {
        let mut goban = Goban::new(9, 9);
        goban.play(Point::new(1, 1), Color::Black);
        goban.play(Point::new(2, 1), Color::Black);

        assert_eq!(goban.num_liberties(Point::new(1, 1)), 3);
    }
}
