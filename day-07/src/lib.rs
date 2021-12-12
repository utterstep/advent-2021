use std::{error::Error, str::FromStr};

use advent_utils::{Part, Solver};

#[derive(Debug)]
pub struct Solution {
    crab_positions: Vec<i64>,
    min_pos: i64,
    max_pos: i64,
}

impl FromStr for Solution {
    type Err = Box<dyn Error>;

    fn from_str(input_data: &str) -> Result<Self, Self::Err> {
        let crab_positions = input_data
            .trim_end()
            .split(',')
            .map(str::parse)
            .collect::<Result<_, _>>()?;

        let mut min_pos = i64::MAX;
        let mut max_pos = 0;

        for &pos in &crab_positions {
            if pos < min_pos {
                min_pos = pos;
            }
            if pos > max_pos {
                max_pos = pos;
            }
        }

        Ok(Self {
            crab_positions,
            min_pos,
            max_pos,
        })
    }
}

#[inline]
fn sum_up_to_n(n: i64) -> i64 {
    (n * (n + 1)) / 2
}

impl Solver for Solution {
    fn solve(&self, part: Part) -> String {
        match part {
            Part::One => {
                let mut min_fuel = i64::MAX;

                for p in self.min_pos..=self.max_pos {
                    let fuel = self
                        .crab_positions
                        .iter()
                        .map(|&pos| (pos - p).abs())
                        .sum::<i64>();

                    if fuel < min_fuel {
                        min_fuel = fuel;
                    }
                }

                format!("min fuel usage is {}", min_fuel)
            }
            Part::Two => {
                let mut min_fuel = i64::MAX;

                for p in self.min_pos..=self.max_pos {
                    let fuel = self
                        .crab_positions
                        .iter()
                        .map(|&pos| sum_up_to_n((pos - p).abs()))
                        .sum::<i64>();

                    if fuel < min_fuel {
                        min_fuel = fuel;
                    }
                }

                format!("min fuel usage is {}", min_fuel)
            }
        }
    }

    fn day_number() -> u32 {
        7
    }
}
