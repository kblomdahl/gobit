use crate::{Color, Point};

#[derive(Clone)]
pub struct Block {
    color: Color,
    head: Point,
}

impl Block {
    pub fn new(head: Point, color: Color) -> Self {
        Self { head, color }
    }

    pub fn head(&self) -> Point {
        self.head
    }

    pub fn color(&self) -> Color {
        self.color
    }
}
