use crate::Point;

#[derive(Clone, Copy)]
pub struct Vertex {
    next_link: Point,
    block: u16,
}

impl Vertex {
    const EMPTY: u16 = u16::MAX - 1;
    const INVALID: u16 = u16::MAX;

    pub fn empty(at: Point) -> Self {
        Self {
            next_link: at,
            block: Self::EMPTY,
        }
    }

    pub fn invalid() -> Self {
        Self {
            next_link: Point::new(0, 0),
            block: Self::INVALID,
        }
    }

    pub fn is_empty(&self) -> bool {
        self.block == Self::EMPTY
    }

    pub fn is_valid(&self) -> bool {
        self.block != Self::INVALID
    }

    pub fn next_link(&self) -> Point {
        self.next_link
    }

    pub fn set_next_link(&mut self, next_link: Point) {
        self.next_link = next_link;
    }

    pub fn block(&self) -> usize {
        self.block as usize
    }

    pub fn set_block(&mut self, block: usize) {
        self.block = block as u16;
    }
}
