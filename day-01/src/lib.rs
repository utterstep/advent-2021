use std::{error::Error, str::FromStr};

use advent_utils::{parse_raw_data, Part, Solver};

#[derive(Debug)]
pub struct Solution {
    depths: Vec<u32>,
}

impl FromStr for Solution {
    type Err = Box<dyn Error>;

    fn from_str(input_data: &str) -> Result<Self, Self::Err> {
        let depths = parse_raw_data(input_data)?;

        Ok(Self { depths })
    }
}

impl Solver for Solution {
    fn solve(&self, part: Part) -> String {
        match part {
            Part::One => {
                let n_increases = self
                    .depths
                    .windows(2)
                    .filter(|&window| window[0] < window[1])
                    .count();

                format!("number of times depth increased is {}", n_increases)
            }
            Part::Two => {
                let n_increasing_winows = self
                    .depths
                    .windows(4)
                    .filter(|&window| window[0] < window[3])
                    .count();

                format!(
                    "number of times three-measurement sliding window sum increased is {}",
                    n_increasing_winows
                )
            }
        }
    }

    fn day_number() -> u32 {
        1
    }
}
