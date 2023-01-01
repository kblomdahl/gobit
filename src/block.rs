use crate::{Color, Point};

#[derive(Clone)]
pub struct Block {
    color: Color,
    head: Point,
    num_liberties: u8,
    hash: u32
}

impl Block {
    pub fn new(head: Point, color: Color, num_liberties: u8, hash: u32) -> Self {
        Self { head, color, num_liberties, hash }
    }

    pub fn head(&self) -> Point {
        self.head
    }

    pub fn color(&self) -> Color {
        self.color
    }

    pub fn num_liberties(&self) -> u8 {
        self.num_liberties
    }

    pub fn inc_num_liberties(&mut self) {
        self.num_liberties += 1;
    }

    pub fn dec_num_liberties(&mut self) {
        self.num_liberties -= 1;
    }

    pub fn hash(&self) -> u32 {
        self.hash
    }

    pub fn update_hash(&mut self, other_hash: u32) {
        self.hash ^= other_hash;
    }
}
