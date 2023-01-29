use crate::{Color, Goban, Point, pattern::{Eye, Pattern, Searcher, SearchStep}};
use rand::prelude::*;
use std::{iter, fmt::{Debug, Display}, cmp::Ordering};

const EXPANSION_LIMIT: u32 = 8;

#[derive(PartialEq)]
struct OrderedFloat(f32);

impl Display for OrderedFloat {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", self.0)
    }
}

impl PartialOrd for OrderedFloat {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        if self.0 > other.0 {
            Some(Ordering::Greater)
        } else if self.0 == other.0 {
            Some(Ordering::Equal)
        } else {
            Some(Ordering::Less)
        }
    }
}

impl Ord for OrderedFloat {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.partial_cmp(other).unwrap()
    }
}

impl Eq for OrderedFloat {
    // pass
}

struct Candidate {
    at: Point,
    sims: u32,
    wins: u32,
    child: Option<Box<SearchTree>>,
}

impl Candidate {
    const PASS: (u8, u8) = (127u8, 127u8);

    fn new(at: Point) -> Self {
        Self {
            at,
            sims: 0,
            wins: 0,
            child: None
        }
    }

    fn pass() -> Self {
        Self::new(Self::PASS.into())
    }

    fn is_pass(&self) -> bool {
        self.at == Self::PASS.into()
    }

    /*
    fn ucb1(&self, total_sims: u32) -> u32 {
        const SHIFT_LEFT: u32 = 8;
        let ln_total_sims = if total_sims == 0 { 0 } else { total_sims.ilog2() };

        if self.sims == 0 {
            1 << (SHIFT_LEFT - 1) + isqrt(ln_total_sims << (SHIFT_LEFT * 2))
        } else {
            let win_pct = (self.wins << SHIFT_LEFT) / self.sims;
            let ln_total_sims_pct = (ln_total_sims << SHIFT_LEFT) / self.sims;

            win_pct + isqrt(ln_total_sims_pct << SHIFT_LEFT)
        }
    }
    */

    fn ucb1(&self, total_sims: u32) -> OrderedFloat {
        const C: f32 = 1.41;
        let ln_total_sims = if total_sims == 0 { 0.0 } else { (total_sims as f32).ln() };

        OrderedFloat(if self.sims == 0 {
            0.5 + ln_total_sims.sqrt()
        } else {
            let win_pct = self.wins as f32 / self.sims as f32;
            let ln_total_sims_pct = ln_total_sims / self.sims as f32;

            win_pct + C * ln_total_sims_pct.sqrt()
        })
    }
}

fn isqrt(mut n: u32) -> u32 {
    if n == 0 {
        return 0;
    }

    // Compute bit, the largest power of 4 <= n
    let max_shift: u32 = 0u32.leading_zeros() - 1;
    let shift: u32 = (max_shift - n.leading_zeros()) & !1;

    // https://en.wikipedia.org/wiki/Methods_of_computing_square_roots#Binary_numeral_system_(base_2)
    let mut bit = 1u32 << shift;
    let mut result = 0u32;

    while bit != 0 {
        if n >= (result + bit) {
            n = n - (result + bit);
            result = (result >> 1) + bit;
        } else {
            result = result >> 1;
        }
        bit = bit >> 2;
    }

    result
}

pub struct ProbeResult {
    goban: Goban,
    black: u16,
    white: u16,
    komi: f32
}

impl ProbeResult {
    fn playout(mut goban: Goban, mut to_move: Color, komi: f32) -> Self {
        let mut pass_count = 0;

        while pass_count < 2 {
            let mut eye_searcher = Eye::new(to_move).into_searcher(&goban);
            let candidate = iter::from_fn(|| {
                loop {
                    match eye_searcher.next() {
                        SearchStep::Done => { return None },
                        SearchStep::Reject(at) if goban.is_legal(at, to_move) => { return Some(at) },
                        _ => { },
                    }
                }
            }).choose(&mut thread_rng());

            if let Some(at) = candidate {
                goban.play(at, to_move);
                pass_count = 0;
            } else {
                pass_count += 1;
            }

            to_move = to_move.opposite();
        }

        Self::score(goban, komi)
    }

    fn score(goban: Goban, komi: f32) -> Self {
        let mut black = 0;
        let mut white = 0;

        for at in goban.iter() {
            match goban.at(at) {
                None => {
                    let neighbour = at.neighbours().filter(|&n| goban[n].is_valid()).next().unwrap();

                    match goban.at(neighbour) {
                        None => { /* pass */ },
                        Some(Color::Black) => { black += 1; },
                        Some(Color::White) => { white += 1; },
                    }
                },
                Some(Color::Black) => { black += 1; },
                Some(Color::White) => { white += 1; },
            }
        }

        Self { goban, black, white, komi }
    }

    pub fn goban(&self) -> &Goban {
        &self.goban
    }

    pub fn winner(&self) -> Color {
        if self.black as f32 > self.white as f32 + self.komi {
            Color::Black
        } else {
            Color::White
        }
    }
}

pub struct SearchTree {
    candidates: Vec<Candidate>,
    pass_count: u8,
    total_sims: u32,
    to_move: Color,
}

impl Debug for SearchTree {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "sims: {}, pass: {}, to_move: {}", self.total_sims, self.pass_count, match self.to_move {
            Color::Black => "black",
            Color::White => "white",
        })?;

        let mut candidates = self.candidates.iter().collect::<Vec<_>>();
        candidates.sort_unstable_by_key(|cand| -(cand.sims as i64));

        for cand in candidates.iter().take(10) {
            let description = if cand.is_pass() {
                "pass".into()
            } else {
                let (x, y): (u8, u8) = cand.at.into();

                format!(
                    "{}{}",
                    char::from_u32('a' as u32 + x as u32).unwrap(),
                    y + 1,
                )
            };
            let win_pct = cand.wins as f32 / cand.sims as f32;

            writeln!(
                f,
                "{:7} / {:5} ({:.2}) {} (ucb1 {})",
                cand.sims,
                cand.wins,
                if win_pct.is_finite() { win_pct } else { 0.0 },
                description,
                cand.ucb1(self.total_sims),
            )?;
        }

        Ok(())
    }
}

impl SearchTree {
    pub fn new(goban: &Goban, to_move: Color, pass_count: u8) -> Self {
        Self {
            candidates: goban.iter()
                .filter(|&at| goban.is_legal(at, to_move))
                .map(|at| Candidate::new(at))
                .chain([Candidate::pass()].into_iter())
                .collect(),
            pass_count,
            total_sims: 0,
            to_move,
        }
    }

    fn as_sgf_tree(&self) -> String {
        const LETTERS: [char; 26] = [
            'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm',
            'n', 'o', 'p', 'q', 'r', 's', 't', 'u', 'v', 'w', 'x', 'y', 'z'
        ];

        self.candidates.iter()
            .filter(|cand| cand.sims > 0)
            .map(|cand| {
                let color = match self.to_move {
                    Color::Black => 'B',
                    Color::White => 'W',
                };
                let coordinate = if cand.is_pass() {
                    "".into()
                } else {
                    let (x, y): (usize, usize) = cand.at.into();

                    format!("{}{}", LETTERS[x], LETTERS[y])
                };
                let comment = format!(
                    "{} ({} / {})",
                    color,
                    cand.wins,
                    cand.sims,
                );

                if let Some(next_child) = &cand.child {
                    format!("(;{}[{}]C[{}]{})", color, coordinate, comment, next_child.as_sgf_tree())
                } else {
                    format!("(;{}[{}]C[{}])", color, coordinate, comment)
                }
            })
            .collect::<String>()
    }

    fn as_sgf_stone_list(&self, goban: &Goban, property_name: &str, color: Color) -> String {
        const LETTERS: [char; 26] = [
            'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm',
            'n', 'o', 'p', 'q', 'r', 's', 't', 'u', 'v', 'w', 'x', 'y', 'z'
        ];

        let black_list = goban.iter()
            .filter(|&at| goban.at(at) == Some(color))
            .map(|at| {
                let (x, y): (usize, usize) = at.into();

                format!("[{}{}]", LETTERS[x], LETTERS[y])
            })
            .collect::<String>();

        if black_list.len() > 0 {
            format!("{}{}", property_name, black_list)
        } else {
            "".into()
        }
    }

    pub fn as_sgf(&self, goban: &Goban) -> String {
        format!(
            "(FF[3]SZ[{}];{}{}{})",
            goban.width().max(goban.height()),
            self.as_sgf_stone_list(goban, "AB", Color::Black),
            self.as_sgf_stone_list(goban, "AW", Color::White),
            self.as_sgf_tree()
        )
    }

    pub fn winner(&self) -> Color {
        let most_sims = self.candidates.iter()
            .max_by_key(|cand| cand.sims)
            .unwrap();

        if most_sims.wins > most_sims.sims / 2 {
            self.to_move
        } else {
            self.to_move.opposite()
        }
    }

    fn to_move(&self) -> Color {
        self.to_move
    }

    fn next_candidate<'a>(&'a mut self) -> &'a mut Candidate {
        let total_sims = self.total_sims;

        self.candidates.iter_mut()
            .max_by_key(|cand| (cand.ucb1(total_sims), thread_rng().next_u32()))
            .unwrap()
    }

    fn probe_candidate(
        candidate: &mut Candidate,
        goban: Goban,
        to_move: Color,
        pass_count: u8,
        komi: f32,
    ) -> ProbeResult
    {
        let probe_result = if pass_count >= 2 {
            ProbeResult::score(goban, komi)
        } else if let Some(next_child) = candidate.child.as_mut() {
            next_child.probe(goban, komi)
        } else if candidate.sims >= EXPANSION_LIMIT {
            candidate.child = Some(Box::new(SearchTree::new(&goban, to_move.opposite(), pass_count)));
            candidate.child.as_mut().unwrap().probe(goban, komi)
        } else {
            ProbeResult::playout(goban, to_move, komi)
        };

        candidate.wins += if probe_result.winner() == to_move { 1 } else { 0 };
        candidate.sims += 1;

        probe_result
    }

    pub fn probe(&mut self, mut goban: Goban, komi: f32) -> ProbeResult {
        let to_move = self.to_move();
        let pass_count = self.pass_count;
        let next_candidate = self.next_candidate();
        let result = if !next_candidate.is_pass() {
            goban.play(next_candidate.at, to_move);
            Self::probe_candidate(next_candidate, goban, to_move, 0, komi)
        } else {
            Self::probe_candidate(next_candidate, goban, to_move, pass_count + 1, komi)
        };

        self.total_sims += 1;

        result
    }
}
