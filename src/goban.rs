use crate::{Color, Point, array2d::Array2D, vertex::Vertex, block::Block, zobrist, ring::Ring};
use slab::Slab;
use std::{ops::{Index, IndexMut}, iter};

#[derive(Clone)]
pub struct Goban {
    vertices: Array2D<Vertex>,
    blocks: Slab<Block>,
    super_ko: Ring<u32, 8>,
    hash: u32,
}

impl Eq for Goban {
    // pass
}

impl PartialEq for Goban {
    fn eq(&self, other: &Self) -> bool {
        self.hash == other.hash
    }
}

impl Index<Point> for Goban {
    type Output = Vertex;

    fn index(&self, index: Point) -> &Self::Output {
        &self.vertices[(index.x(), index.y())]
    }
}

impl IndexMut<Point> for Goban {
    fn index_mut(&mut self, index: Point) -> &mut Self::Output {
        &mut self.vertices[(index.x(), index.y())]
    }
}

impl Goban {
    /// Returns an empty board of the given width `width` and height `height`.
    ///
    /// # Arguments
    ///
    /// * `width` -
    /// * `height` -
    ///
    pub fn new(width: usize, height: usize) -> Self {
        let mut goban = Self {
            vertices: Array2D::new(width + 2, height + 2, Vertex::invalid()),
            blocks: Slab::new(),
            super_ko: Ring::new(zobrist::empty()),
            hash: zobrist::empty(),
        };

        for point in goban.iter() {
            goban[point] = Vertex::empty(point);
        }

        goban
    }

    /// Returns the width of the board.
    pub fn width(&self) -> usize {
        self.vertices.width() - 2
    }

    /// Returns the height of the board.
    pub fn height(&self) -> usize {
        self.vertices.height() - 2
    }

    /// Returns an iterator over all points of the board.
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

    /// Returns the color of the stone at the given point `at`, or `None` if
    /// the vertex is empty.
    pub fn at(&self, at: Point) -> Option<Color> {
        if self[at].is_empty() || !self[at].is_valid() {
            None
        } else {
            Some(self.block_at(at).color())
        }
    }

    fn block_at(&self, at: Point) -> &Block {
        self.block_by(self[at].block())
    }

    fn block_by(&self, block: usize) -> &Block {
        unsafe { self.blocks.get_unchecked(block) }
    }

    fn block_by_mut(&mut self, block: usize) -> &mut Block {
        unsafe { self.blocks.get_unchecked_mut(block) }
    }

    fn is_super_ko(&self, hash: u32) -> bool {
        self.super_ko.contains(hash)
    }

    /// Returns if playing a stone at the given point `at` and color `color` is
    /// a legal move according to the rules.
    ///
    /// # Arguments
    ///
    /// * `at` -
    /// * `color` -
    ///
    pub fn is_legal(&self, at: Point, color: Color) -> bool {
        self[at].is_valid() && self[at].is_empty() && {
            let opposite = color.opposite();
            let mut hash = zobrist::hash(at, color);
            let mut is_legal = false;

            for other in at.neighbours() {
                if !self[other].is_valid() {
                    // pass
                } else if self[other].is_empty() || (self.block_at(other).color() == color && self.block_at(other).num_liberties() >= 2) {
                    is_legal = true;
                } else if self.block_at(other).color() == opposite && self.block_at(other).num_liberties() == 1 {
                    hash ^= self.block_at(other).hash();
                    is_legal = true;
                }
            }

            is_legal && !self.is_super_ko(self.hash ^ hash)
        }
    }

    fn capture_single_at(&mut self, at: Point) {
        let opposite = self.block_at(at).color().opposite();
        let mut visited = [usize::MAX; 4];
        let mut n = 0;

        for other in at.neighbours() {
            if !self[other].is_valid() || self[other].is_empty() {
                // pass
            } else {
                let other_block = self[other].block();

                if self.block_at(other).color() == opposite && !visited[0..n].contains(&other_block) {
                    visited[n] = other_block;
                    n += 1;

                    self.block_by_mut(other_block).inc_num_liberties();
                }
            }
        }

        self[at] = Vertex::empty(at);
    }

    fn capture_at(&mut self, at: Point) -> u32 {
        let mut curr = at;
        let block = self[curr].block();
        let hash = self.block_by(block).hash();

        loop {
            let next_link = self[curr].next_link();
            self.capture_single_at(curr);
            curr = next_link;
            if curr == at {
                break
            }
        }

        self.blocks.remove(block);
        hash
    }

    fn is_liberty_of(&self, liberty: Point, block: usize) -> bool {
        for other in liberty.neighbours() {
            if self[other].block() == block {
                return true
            }
        }

        false
    }

    fn connect_single_with(&mut self, at: Point, to_block: usize) {
        for other in at.neighbours() {
            if self[other].is_empty() && self[other].is_valid() {
                if !self.is_liberty_of(other, to_block) {
                    self.block_by_mut(to_block).inc_num_liberties();
                }
            }
        }

        self[at].set_block(to_block);

        // move `at` to just after the head of the `to_block` in the cyclic
        // list of vertices:
        //
        // 1 -> 2 -> .. -> 1 (cyclic)
        // a
        //
        // becomes
        //
        // 1 -> a -> 2 -> .. -> 1 (cyclic)
        //
        let to_head = self.block_by(to_block).head();
        let to_head_next = self[to_head].next_link();

        self[to_head].set_next_link(at);
        self[at].set_next_link(to_head_next);
    }

    fn connect_with(&mut self, at: Point, to: Point) {
        let a_block = self[at].block();
        let b_block = self[to].block();

        if a_block == b_block {
            return
        }

        let mut curr = at;
        let a_hash = self.block_by(a_block).hash();

        loop {
            let next_link = self[curr].next_link();
            self.connect_single_with(curr, b_block);

            curr = next_link;
            if curr == at {
                break
            }
        }

        self.block_by_mut(b_block).dec_num_liberties();
        self.block_by_mut(b_block).update_hash(a_hash);
        self.blocks.remove(a_block);
    }

    fn play_update_neighbours(&mut self, at: Point, color: Color) {
        let opposite = color.opposite();
        let mut visited = [usize::MAX; 4];
        let mut n = 0;

        for other in at.neighbours() {
            if self[other].is_empty() || !self[other].is_valid() {
                // pass
            } else if self.block_at(other).color() == opposite {
                let other_block = self[other].block();

                if !visited[0..n].contains(&other_block) {
                    visited[n] = other_block;
                    n += 1;

                    if self.block_at(other).num_liberties() == 1 {
                        self.hash ^= self.capture_at(other);
                    } else  {
                        self.block_by_mut(other_block).dec_num_liberties();
                    }
                }
            } else if self.block_at(other).color() == color {
                self.connect_with(at, other);
            }
        }
    }

    /// Play a stone at the given vertex `at` of color `color`. This function
    /// assumes that the given move is valid, and the result is undefined if it
    /// is not.
    ///
    /// # Arguments
    ///
    /// * `at` -
    /// * `color` -
    ///
    pub fn play(&mut self, at: Point, color: Color) {
        debug_assert!(self.is_legal(at, color));

        let block = self.blocks.insert(
            Block::new(
                at,
                color,
                at.neighbours().filter(|&other| self[other].is_empty() && self[other].is_valid()).count() as u8,
                zobrist::hash(at, color),
            )
        );

        self[at].set_block(block);
        self[at].set_next_link(at);
        self.hash ^= zobrist::hash(at, color);
        self.play_update_neighbours(at, color);
        self.super_ko.insert(self.hash);
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

    /// ```
    /// x x
    /// ```
    #[test]
    fn play_fills_vertices() {
        let mut goban = Goban::new(9, 9);
        goban.play(Point::new(1, 1), Color::Black);
        goban.play(Point::new(2, 1), Color::Black);

        assert_eq!(goban.iter().filter(|at| goban.at(*at) == Some(Color::Black)).count(), 2);
    }

    /// ```
    /// o x
    /// x
    /// ```
    #[test]
    fn play_clears_captured_vertices() {
        let mut goban = Goban::new(9, 9);
        goban.play(Point::new(1, 1), Color::White);
        goban.play(Point::new(1, 2), Color::Black);
        goban.play(Point::new(2, 1), Color::Black);

        assert_eq!(goban.iter().filter(|at| goban.at(*at) == Some(Color::Black)).count(), 2);
        assert_eq!(goban.iter().filter(|at| goban.at(*at) == Some(Color::White)).count(), 0);
    }

    /// ```
    /// x x
    /// ```
    #[test]
    fn capture_at_clears_all_vertices() {
        let mut goban = Goban::new(9, 9);
        goban.play(Point::new(1, 1), Color::Black);
        goban.play(Point::new(2, 1), Color::Black);
        goban.capture_at(Point::new(1, 1));

        assert_eq!(goban.iter().filter(|at| goban.at(*at) == Some(Color::Black)).count(), 0);
    }

    /// ```
    /// x x
    /// ```
    #[test]
    fn has_exactly_n_liberties() {
        let mut goban = Goban::new(9, 9);
        goban.play(Point::new(1, 1), Color::Black);
        goban.play(Point::new(2, 1), Color::Black);

        assert_eq!(goban.block_at(Point::new(1, 1)).num_liberties(), 3);
    }

    /// ```
    /// x x x
    /// x   x
    /// x o x
    /// ```
    #[test]
    fn play_does_not_double_increase_liberties() {
        let mut goban = Goban::new(9, 9);
        goban.play(Point::new(1, 1), Color::White);
        goban.play(Point::new(1, 2), Color::White);
        goban.play(Point::new(1, 3), Color::White);
        goban.play(Point::new(2, 3), Color::White);
        goban.play(Point::new(3, 1), Color::White);
        goban.play(Point::new(3, 2), Color::White);
        goban.play(Point::new(3, 3), Color::White);
        goban.play(Point::new(2, 1), Color::Black);

        assert_eq!(goban.block_at(Point::new(3, 3)).num_liberties(), 7);
        assert_eq!(goban.block_at(Point::new(2, 1)).num_liberties(), 1);
    }

    /// ```
    /// x o x
    /// o x
    /// ```
    #[test]
    fn is_legal_detects_super_ko() {
        let mut goban = Goban::new(9, 9);
        goban.play(Point::new(1, 1), Color::Black);
        goban.play(Point::new(2, 2), Color::Black);
        goban.play(Point::new(3, 1), Color::Black);
        goban.play(Point::new(1, 2), Color::White);
        goban.play(Point::new(2, 1), Color::White);

        assert_eq!(goban.at(Point::new(1, 1)), None);
        assert!(!goban.is_legal(Point::new(1, 1), Color::Black));
    }
}
