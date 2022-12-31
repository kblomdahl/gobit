use crate::{Color, Point};

#[derive(Clone)]
pub struct Block {
    color: Color,
    head: Point,
    num_liberties: usize,
}

impl Block {
    pub fn new(head: Point, color: Color, num_liberties: usize) -> Self {
        Self { head, color, num_liberties }
    }

    pub fn head(&self) -> Point {
        self.head
    }

    pub fn color(&self) -> Color {
        self.color
    }

    pub fn num_liberties(&self) -> usize {
        self.num_liberties
    }

    pub fn inc_num_liberties(&mut self) {
        self.num_liberties += 1;
    }

    pub fn dec_num_liberties(&mut self) {
        self.num_liberties -= 1;
    }
}
