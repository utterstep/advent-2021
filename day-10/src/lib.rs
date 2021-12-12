use std::{error::Error, str::FromStr};

use advent_utils::{Part, Solver};

#[derive(Debug)]
pub struct Solution {
    line_scores: Vec<Score>,
}

impl FromStr for Solution {
    type Err = Box<dyn Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self {
            line_scores: s.lines().map(calc_line_score).collect(),
        })
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Score {
    Ok,
    Incomplete(u64),
    Corrupted(u64),
}

fn calc_line_score(line: &str) -> Score {
    let mut stack = vec![];

    for c in line.bytes() {
        match c {
            b'<' | b'(' | b'{' | b'[' => stack.push(c),
            b'>' => match stack.pop() {
                Some(b'<') => {}
                _ => return Score::Corrupted(25137),
            },
            b'}' => match stack.pop() {
                Some(b'{') => {}
                _ => return Score::Corrupted(1197),
            },
            b']' => match stack.pop() {
                Some(b'[') => {}
                _ => return Score::Corrupted(57),
            },
            b')' => match stack.pop() {
                Some(b'(') => {}
                _ => return Score::Corrupted(3),
            },
            _ => unreachable!(),
        }
    }

    if stack.is_empty() {
        Score::Ok
    } else {
        Score::Incomplete(stack.iter().rev().fold(0, |score, c| {
            score * 5
                + match c {
                    b'(' => 1,
                    b'[' => 2,
                    b'{' => 3,
                    b'<' => 4,
                    _ => unreachable!(),
                }
        }))
    }
}

impl Solver for Solution {
    fn solve(&self, part: Part) -> String {
        match part {
            Part::One => {
                format!(
                    "total corruptness score is {}",
                    self.line_scores
                        .iter()
                        .filter_map(|score| match score {
                            Score::Corrupted(s) => Some(s),
                            _ => None,
                        })
                        .sum::<u64>(),
                )
            }
            Part::Two => {
                let mut autocomplete_scores = self
                    .line_scores
                    .iter()
                    .filter_map(|score| match score {
                        Score::Incomplete(s) => Some(*s),
                        _ => None,
                    })
                    .collect::<Vec<_>>();

                autocomplete_scores.sort_unstable();

                format!(
                    "winner is {}",
                    autocomplete_scores[autocomplete_scores.len() / 2]
                )
            }
        }
    }

    fn day_number() -> u32 {
        10
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_part_one() {
        let lines = include_str!("../short.txt").split('\n');

        assert_eq!(
            lines
                .filter_map(|line| match calc_line_score(line) {
                    Score::Corrupted(s) => Some(s),
                    _ => None,
                })
                .sum::<u64>(),
            26397
        )
    }

    #[test]
    fn test_smallest_example_part_two() {
        let line = "<{([{{}}[<[[[<>{}]]]>[]]";

        assert_eq!(calc_line_score(line), Score::Incomplete(294))
    }

    #[test]
    fn test_example_part_two() {
        let lines = include_str!("../short.txt").split('\n');

        let autocomplete_scores = lines
            .filter_map(|line| match calc_line_score(line) {
                Score::Incomplete(s) => Some(s),
                _ => None,
            })
            .collect::<Vec<_>>();

        assert_eq!(
            autocomplete_scores,
            vec![288957, 5566, 1480781, 995444, 294]
        )
    }
}
