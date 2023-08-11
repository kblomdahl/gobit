use crate::{Color, Goban, Point};
use std::{fmt::{Debug, Display}, cmp::Ordering};
use statrs::distribution::{Normal, ContinuousCDF};

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

struct CandidateStatistics {
    mean: f32,
    count: f32,
    m2: f32,
}

impl CandidateStatistics {
    fn new() -> Self {
        Self {
            mean: 0.0,
            count: 0.0,
            m2: 0.0
        }
    }

    fn update(&mut self, new_value: f32) {
        let delta = new_value - self.mean;

        self.count += 1.0;
        self.mean += delta / self.count;
        self.m2 += delta * (new_value - self.mean);
    }

    fn mean(&self) -> f32 {
        self.mean
    }

    fn count(&self) -> f32 {
        self.count
    }

    fn variance(&self) -> f32 {
        self.m2 / (self.count - 1.0)
    }
}

struct Candidate {
    at: Point,
    stats: CandidateStatistics,
    child: Option<Box<SearchTree>>,
}

impl Candidate {
    const PASS: (u8, u8) = (127u8, 127u8);

    fn new(at: Point) -> Self {
        Self {
            at,
            stats: CandidateStatistics::new(),
            child: None
        }
    }

    fn pass() -> Self {
        Self::new(Self::PASS.into())
    }

    fn is_pass(&self) -> bool {
        self.at == Self::PASS.into()
    }

    fn update(&mut self, prob: f32) {
        self.stats.update(prob);
    }

    fn mean(&self) -> f32 {
        self.stats.mean()
    }

    fn wins(&self) -> f32 {
        self.stats.mean() * self.stats.count()
    }

    fn sims(&self) -> f32 {
        self.stats.count()
    }

    fn variance(&self) -> f32 {
        self.stats.variance()
    }

    fn ucb1(&self, total_sims: u32) -> OrderedFloat {
        const C: f32 = 4.0;
        let ln_total_sims = if total_sims == 0 { 0.0 } else { (total_sims as f32).ln() };

        OrderedFloat(if self.sims() == 0.0 {
            0.5 + ln_total_sims.sqrt()
        } else {
            let win_pct = self.wins() / self.sims();
            let ln_total_sims_pct = ln_total_sims / self.sims();

            win_pct + C * ln_total_sims_pct.sqrt()
        })
    }
}

pub struct ProbeResult {
    black: u16,
    white: u16,
    undecided: u16,
    komi: f32
}

impl ProbeResult {
    fn score(goban: Goban, komi: f32) -> Self {
        let mut black = 0;
        let mut white = 0;
        let mut undecided = 0;

        for at in goban.iter() {
            match goban.at(at) {
                None => {
                    let mut black_neighbours = 0;
                    let mut white_neighbours = 0;

                    for neighbour in at.neighbours().filter(|&n| goban[n].is_valid()) {
                        match goban.at(neighbour) {
                            Some(Color::Black) => { black_neighbours += 1 },
                            Some(Color::White) => { white_neighbours += 1 },
                            None => {},
                        }
                    }

                    if black_neighbours + white_neighbours == 0 {
                        undecided += 1;
                    } else if black_neighbours == black_neighbours + white_neighbours {
                        black += 1;
                    } else if white_neighbours == black_neighbours + white_neighbours {
                        white += 1;
                    } else {
                        undecided += 1;
                    }
                },
                Some(Color::Black) => { black += 1; },
                Some(Color::White) => { white += 1; },
            }
        }

        Self { black, white, undecided, komi }
    }

    pub fn winner(&self) -> (Color, f32) {
        if self.undecided == 0 {
            if (self.black as f32) > self.white as f32 + self.komi {
                (Color::Black, 1.0)
            } else {
                (Color::White, 1.0)
            }
        } else {
            let mean = self.white as f64 + self.komi as f64 - self.black as f64;
            let std = 2.0 * self.undecided as f64 / 12.0;
            let black_prob = Normal::new(mean, std).unwrap();

            (Color::Black, black_prob.cdf(0.0) as f32)
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
        candidates.sort_unstable_by_key(|cand| OrderedFloat(-cand.sims()));

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
            let win_pct = cand.wins() / cand.sims();

            writeln!(
                f,
                "{:7} / {:5} ({:.2} +/- {:.2}) {} (ucb1 {})",
                cand.sims(),
                cand.wins(),
                if win_pct.is_finite() { win_pct } else { 0.0 },
                cand.variance().sqrt(),
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

        let mut candidates = self.candidates.iter()
            .filter(|cand| cand.sims() > 0.0)
            .collect::<Vec<_>>();
        candidates.sort_unstable_by_key(|cand| -(cand.sims() as i64));
        candidates.iter()
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
                    "{} ({} / {}) = {} +/- {}",
                    color,
                    cand.wins(),
                    cand.sims(),
                    cand.mean(),
                    cand.variance().sqrt(),
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
            "(FF[3]SZ[{}]{}{};{})",
            goban.width().max(goban.height()),
            self.as_sgf_stone_list(goban, "AB", Color::Black),
            self.as_sgf_stone_list(goban, "AW", Color::White),
            self.as_sgf_tree()
        )
    }

    pub fn total_sims(&self) -> u32 {
        self.total_sims
    }

    pub fn is_done(&self, prob: f32) -> bool {
        let inv_prob2 = (1.0 - prob) * (1.0 - prob);

        self.candidates.iter()
            .all(|cand| cand.sims() > 1.0 && cand.variance() < inv_prob2)
    }

    pub fn winner(&self) -> Color {
        let most_sims = self.candidates.iter()
            .max_by_key(|cand| OrderedFloat(cand.sims()))
            .unwrap();

        if most_sims.mean() > 0.5 {
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
            .max_by_key(|cand| cand.ucb1(total_sims))
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
        } else if candidate.sims() >= 1.0 {
            candidate.child = Some(Box::new(SearchTree::new(&goban, to_move.opposite(), pass_count)));
            candidate.child.as_mut().unwrap().probe(goban, komi)
        } else {
            ProbeResult::score(goban, komi)
        };

        let (winner, prob) = probe_result.winner();
        candidate.update(if winner == to_move { prob } else { 1.0 - prob });

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
