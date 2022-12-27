use crate::Point;

use super::color::Color;

#[derive(Clone, Copy)]
pub struct Vertex {
    is_valid: bool,
    color: Option<Color>,
    next_link: Point,
    head: Point,
    mark: usize,
}

impl Vertex {
    pub fn empty(at: Point) -> Self {
        Self {
            is_valid: true,
            color: None,
            next_link: at,
            head: at,
            mark: usize::MIN,
        }
    }

    pub fn invalid() -> Self {
        Self {
            is_valid: false,
            color: None,
            next_link: Point::new(0, 0),
            head: Point::new(0, 0),
            mark: usize::MIN,
        }
    }

    pub fn is_valid(&self) -> bool {
        self.is_valid
    }

    pub fn is_empty(&self) -> bool {
        self.color == None
    }

    pub fn color(&self) -> Option<Color> {
        self.color
    }

    pub fn set_color(&mut self, color: Option<Color>) {
        self.color = color;
    }

    pub fn next_link(&self) -> Point {
        self.next_link
    }

    pub fn set_next_link(&mut self, next_link: Point) {
        self.next_link = next_link;
    }

    pub fn head(&self) -> Point {
        self.head
    }

    pub fn set_head(&mut self, head: Point) {
        self.head = head;
    }

    pub fn mark(&self) -> usize {
        self.mark
    }

    pub fn set_mark(&mut self, mark: usize) {
        self.mark = mark;
    }
}
