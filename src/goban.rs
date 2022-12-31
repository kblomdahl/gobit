use crate::{Color, Point, array2d::Array2D, vertex::Vertex, block::Block};
use slab::Slab;
use std::{ops::{Index, IndexMut}, iter};

pub struct Goban {
    buf: Array2D<Vertex>,
    blocks: Slab<Block>,
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
            blocks: Slab::new(),
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

    pub fn at(&self, at: Point) -> Option<Color> {
        if self[at].is_empty() || !self[at].is_valid() {
            None
        } else {
            Some(self.blocks[self[at].block()].color())
        }
    }

    fn block_at(&self, at: Point) -> &Block {
        &self.blocks[self[at].block()]
    }

    fn has_exactly_n_liberties<const N: usize>(&self, at: Point) -> bool {
        let mut curr = at;
        let mut liberties = [at; N];
        let mut n = 0;

        loop {
            for other in curr.neighbours() {
                if self[other].is_valid() && self[other].is_empty() {
                    if !liberties[0..n].contains(&other) {
                        if n >= N {
                            return false
                        }

                        liberties[n] = other;
                        n += 1;
                    }
                }
            }

            curr = self[curr].next_link();
            if curr == at {
                break
            }
        }

        n == N
    }

    fn has_n_liberties<const N: usize>(&self, at: Point) -> bool {
        let mut curr = at;
        let mut liberties = [at; N];
        let mut n = 0;

        loop {
            for other in curr.neighbours() {
                if self[other].is_valid() && self[other].is_empty() {
                    if !liberties[0..n].contains(&other) {
                        liberties[n] = other;
                        n += 1;

                        if n >= N {
                            return true
                        }
                    }
                }
            }

            curr = self[curr].next_link();
            if curr == at {
                break
            }
        }

        false
    }

    pub fn is_valid(&self, at: Point, color: Color) -> bool {
        let opposite = color.opposite();

        self[at].is_valid() && self[at].is_empty() && at.neighbours()
            .any(|other| {
                self[other].is_valid()
                    && (
                        self[other].is_empty()
                        || (self.block_at(other).color() == color && self.has_n_liberties::<2>(other))
                        || (self.block_at(other).color() == opposite && self.has_exactly_n_liberties::<1>(other))
                    )
            })
    }

    fn capture_at(&mut self, at: Point) {
        let mut curr = at;
        let block = self[curr].block();

        loop {
            let next_link = self[curr].next_link();
            self[curr] = Vertex::empty(curr);

            curr = next_link;
            if curr == at {
                break
            }
        }

        self.blocks.remove(block);
    }

    fn connect_with(&mut self, at: Point, to: Point) {
        let a_block = self[at].block();
        let b_block = self[to].block();

        if a_block == b_block {
            return
        }

        // move ownership of the entire list at `at` to `to`
        let mut curr = at;

        loop {
            self[curr].set_block(b_block);

            curr = self[curr].next_link();
            if curr == at {
                break
            }
        }

        // given the following two cyclic lists:
        //
        // 1 -> 2 -> .. -> 1 (cyclic)
        // a -> b -> .. -> a (cyclic)
        //
        // becomes
        //
        // 1 -> b -> .. a -> 2 -> .. -> 1
        //
        let a = self.blocks[a_block].head();
        let b = self.blocks[b_block].head();
        let a_next = self[a].next_link();
        let b_next = self[b].next_link();
        self[a].set_next_link(b_next);
        self[b].set_next_link(a_next);

        // remove the `at` block which was connected to `to`
        self.blocks.remove(a_block);
    }

    pub fn play(&mut self, at: Point, color: Color) {
        let opposite = color.opposite();
        let block = self.blocks.insert(Block::new(at, color));

        self[at].set_block(block);
        self[at].set_next_link(at);

        for other in at.neighbours() {
            if self[other].is_empty() || !self[other].is_valid() {
                // pass
            } else if self.block_at(other).color() == opposite && self.has_exactly_n_liberties::<1>(other) {
                self.capture_at(other);
            } else if self.block_at(other).color() == color {
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

        assert_eq!(goban.iter().filter(|at| goban.at(*at) == Some(Color::Black)).count(), 2);
    }

    #[test]
    fn play_clears_captured_vertices() {
        let mut goban = Goban::new(9, 9);
        goban.play(Point::new(1, 1), Color::White);
        goban.play(Point::new(1, 2), Color::Black);
        goban.play(Point::new(2, 1), Color::Black);

        assert_eq!(goban.iter().filter(|at| goban.at(*at) == Some(Color::Black)).count(), 2);
        assert_eq!(goban.iter().filter(|at| goban.at(*at) == Some(Color::White)).count(), 0);
    }

    #[test]
    fn capture_at_clears_all_vertices() {
        let mut goban = Goban::new(9, 9);
        goban.play(Point::new(1, 1), Color::Black);
        goban.play(Point::new(2, 1), Color::Black);
        goban.capture_at(Point::new(1, 1));

        assert_eq!(goban.iter().filter(|at| goban.at(*at) == Some(Color::Black)).count(), 0);
    }

    #[test]
    fn has_exactly_n_liberties_returns_liberties() {
        let mut goban = Goban::new(9, 9);
        goban.play(Point::new(1, 1), Color::Black);
        goban.play(Point::new(2, 1), Color::Black);

        assert!(!goban.has_exactly_n_liberties::<1>(Point::new(1, 1)));
        assert!(!goban.has_exactly_n_liberties::<2>(Point::new(1, 1)));
        assert!(goban.has_exactly_n_liberties::<3>(Point::new(1, 1)));
        assert!(!goban.has_exactly_n_liberties::<4>(Point::new(1, 1)));
    }

    #[test]
    fn has_n_liberties_returns_liberties() {
        let mut goban = Goban::new(9, 9);
        goban.play(Point::new(1, 1), Color::Black);
        goban.play(Point::new(2, 1), Color::Black);

        assert!(goban.has_n_liberties::<1>(Point::new(1, 1)));
        assert!(goban.has_n_liberties::<2>(Point::new(1, 1)));
        assert!(goban.has_n_liberties::<3>(Point::new(1, 1)));
        assert!(!goban.has_n_liberties::<4>(Point::new(1, 1)));
    }
}
