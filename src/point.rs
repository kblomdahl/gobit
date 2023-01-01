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

impl Into<(usize, usize)> for Point {
    fn into(self) -> (usize, usize) {
        (self.x as usize - 1, self.y as usize - 1)
    }
}

impl Into<(u8, u8)> for Point {
    fn into(self) -> (u8, u8) {
        (self.x - 1, self.y - 1)
    }
}

impl Point {
    #[inline]
    pub(super) fn new(x: u8, y: u8) -> Self {
        Self { x, y }
    }

    pub fn neighbours(&self) -> impl Iterator<Item=Point> {
        const DELTA: [u8; 6] = [u8::MAX, 1, 0, 0, u8::MAX, 1];
        let (x, y) = (self.x, self.y);

        (0..4).map(move |i| Self::new(x.wrapping_add(DELTA[i]), y.wrapping_add(DELTA[i+2])))
    }

    pub fn x(&self) -> usize {
        self.x as usize
    }

    pub fn y(&self) -> usize {
        self.y as usize
    }
}
