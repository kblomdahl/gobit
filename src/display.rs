use std::fmt::Display;

use crate::{Goban, Color};

impl Display for Goban {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let (width, height) = (self.width(), self.height());

        write!(f, "╔═")?;
        for _ in 0..width { write!(f, "══")?; }
        write!(f, "╗\n")?;

        for y in 0..height {
            write!(f, "║ ")?;
            for x in 0..width {
                write!(f, "{}", match self.at((x, y).into()) {
                    None => "  ",
                    Some(Color::Black) => "× ",
                    Some(Color::White) => "○ ",
                })?;
            }
            write!(f, "║\n")?;
        }

        write!(f, "╚═")?;
        for _ in 0..width { write!(f, "══")?; }
        write!(f, "╝\n")?;

        for _ in 0..(2 * width.saturating_sub(15)) { write!(f, " ")?; }
        write!(f, "× Black   ○ White")
    }
}
