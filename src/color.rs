#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Color {
    Black,
    White
}

impl Color {
    pub fn opposite(&self) -> Self {
        match self {
            Self::Black => Self::White,
            Self::White => Self::Black,
        }
    }
}
