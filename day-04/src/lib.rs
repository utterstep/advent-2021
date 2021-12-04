use std::{error::Error, str::FromStr};

use advent_utils::{Part, Solver};

use board::Board;

mod board;

#[derive(Debug)]
pub struct Solution {
    calls: Vec<u32>,
    boards: Vec<Board>,
}

impl FromStr for Solution {
    type Err = Box<dyn Error>;

    fn from_str(input_data: &str) -> Result<Self, Self::Err> {
        let (calls, boards) = input_data.split_once("\n\n").expect("invalid input format");

        let calls = calls.split(',').map(str::parse).collect::<Result<_, _>>()?;

        let boards = boards
            .split("\n\n")
            .map(str::parse)
            .collect::<Result<_, _>>()?;

        Ok(Self { calls, boards })
    }
}

impl Solver for Solution {
    fn solve(&self, part: Part) -> String {
        let mut boards = self.boards.clone();

        match part {
            Part::One => {
                for called in &self.calls {
                    for board in &mut boards {
                        if board.process_call(*called) {
                            return format!(
                                "first winner score is {}",
                                called * board.unmarked_sum()
                            );
                        }
                    }
                }

                "no winning boards found :(".to_owned()
            }
            Part::Two => {
                let mut scores = vec![];

                for called in &self.calls {
                    for board in boards.iter_mut() {
                        if board.process_call(*called) {
                            scores.push(called * board.unmarked_sum());
                        }
                    }
                }

                match scores.last() {
                    Some(score) => format!("last winner score is {}", score),
                    None => "no winning boards found :(".to_owned(),
                }
            }
        }
    }

    fn day_number() -> u32 {
        4
    }
}
