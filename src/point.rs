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
            Self::new(self.x.saturating_sub(1), self.y),
            Self::new(self.x.saturating_add(1), self.y),
            Self::new(self.x, self.y.saturating_sub(1)),
            Self::new(self.x, self.y.saturating_add(1)),
        ].into_iter()
    }

    pub fn x(&self) -> usize {
        self.x as usize
    }

    pub fn y(&self) -> usize {
        self.y as usize
    }
}
