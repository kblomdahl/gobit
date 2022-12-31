#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq)]
pub struct Point {
    x: u8,
    y: u8,
}

impl From<(usize, usize)> for Point {
    fn from(value: (usize, usize)) -> Self {
        Self::new(value.0 as u8 + 1, value.1 as u8 + 1)
    }
}

impl From<(u8, u8)> for Point {
    fn from(value: (u8, u8)) -> Self {
        Self::new(value.0 as u8 + 1, value.1 as u8 + 1)
    }
}

impl Point {
    pub(super) fn new(x: u8, y: u8) -> Self {
        Self { x, y }
    }

    pub fn neighbours(&self) -> impl Iterator<Item=Point> {
        [
            Self::new(self.x - 1, self.y),
            Self::new(self.x + 1, self.y),
            Self::new(self.x, self.y - 1),
            Self::new(self.x, self.y + 1),
        ].into_iter()
    }

    pub fn is_neighbour(&self, other: Point) -> bool {
        let dx = (other.x as i8).abs_diff(self.x as i8);
        let dy = (other.y as i8).abs_diff(self.y as i8);

        (dx == 0 && dy == 1) || (dx == 1 && dy == 0)
    }

    pub fn x(&self) -> usize {
        self.x as usize
    }

    pub fn y(&self) -> usize {
        self.y as usize
    }
}
