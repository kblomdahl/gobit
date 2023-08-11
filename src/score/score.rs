use crate::{Goban, Color};
use super::search_tree::SearchTree;
use std::fmt::Debug;

/// Determine the final score of the given game using a small Monte Carlo Tree
/// Search (MCTS).
pub struct Score<'a> {
    goban: &'a Goban,
    search_tree: SearchTree,
}

impl<'a> Debug for Score<'a> {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        writeln!(f, "{:?}", self.search_tree)?;
        writeln!(f, "{}", self.search_tree.as_sgf(self.goban))
    }
}

impl<'a> Score<'a> {
    pub fn new(goban: &'a Goban, to_move: Color, komi: f32) -> Self {
        let mut search_tree = SearchTree::new(goban, to_move, 0);
        loop {
            search_tree.probe(goban.clone(), komi);

            if search_tree.total_sims() > 32_000 || search_tree.is_done(0.51) {
                break
            }
        }

        Self { goban, search_tree }
    }

    pub fn winner(&self) -> Color {
        self.search_tree.winner()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// ```
    /// . x x
    /// x . x
    /// x o x
    /// ```
    #[test]
    fn black_wins_3x3() {
        let mut goban = Goban::new(3, 3);
        goban.play((0u8, 0u8).into(), Color::Black);
        goban.play((0u8, 2u8).into(), Color::Black);
        goban.play((1u8, 0u8).into(), Color::Black);
        goban.play((1u8, 2u8).into(), Color::Black);
        goban.play((2u8, 1u8).into(), Color::Black);
        goban.play((2u8, 2u8).into(), Color::Black);
        goban.play((0u8, 1u8).into(), Color::White);

        for _ in 0..10 {
            let score = Score::new(&goban, Color::Black, 0.5);

            assert_eq!(score.winner(), Color::Black, "{:?}", score);
        }
    }

    /// ```
    /// o x x . x . x . .
    /// o x . x o x x x x
    /// o x x o o o o o o
    /// o o o o . . . o o
    /// ```
    #[test]
    fn white_wins_10x5() {
        let mut goban = Goban::new(9, 4);
        goban.play((0u8, 0u8).into(), Color::White);
        goban.play((0u8, 1u8).into(), Color::White);
        goban.play((0u8, 2u8).into(), Color::White);
        goban.play((0u8, 3u8).into(), Color::White);
        goban.play((1u8, 0u8).into(), Color::Black);
        goban.play((1u8, 1u8).into(), Color::Black);
        goban.play((1u8, 2u8).into(), Color::Black);
        goban.play((1u8, 3u8).into(), Color::White);
        goban.play((2u8, 0u8).into(), Color::Black);
        goban.play((2u8, 2u8).into(), Color::Black);
        goban.play((2u8, 3u8).into(), Color::White);
        goban.play((3u8, 1u8).into(), Color::Black);
        goban.play((3u8, 2u8).into(), Color::White);
        goban.play((3u8, 3u8).into(), Color::White);
        goban.play((4u8, 0u8).into(), Color::Black);
        goban.play((4u8, 1u8).into(), Color::White);
        goban.play((4u8, 2u8).into(), Color::White);
        goban.play((5u8, 1u8).into(), Color::Black);
        goban.play((5u8, 2u8).into(), Color::White);
        goban.play((6u8, 0u8).into(), Color::Black);
        goban.play((6u8, 1u8).into(), Color::Black);
        goban.play((6u8, 2u8).into(), Color::White);
        goban.play((7u8, 1u8).into(), Color::Black);
        goban.play((7u8, 2u8).into(), Color::White);
        goban.play((7u8, 3u8).into(), Color::White);
        goban.play((8u8, 1u8).into(), Color::Black);
        goban.play((8u8, 2u8).into(), Color::White);
        goban.play((8u8, 3u8).into(), Color::White);

        for _ in 0..10 {
            let score = Score::new(&goban, Color::White, 0.5);

            assert_eq!(score.winner(), Color::White, "{:?}", score);
        }
    }
}
