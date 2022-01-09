use std::{error::Error, str::FromStr};

use advent_utils::{parse_raw_data, Part, Solver};

use node::Node;
use sum::sum;

mod node;
mod reduce;
mod sum;
mod visitors;

#[derive(Debug)]
pub struct Solution {
    numbers: Vec<String>,
}

impl Solver for Solution {
    fn solve(&self, part: Part) -> String {
        match part {
            Part::One => {
                let mut numbers = self
                    .numbers
                    .iter()
                    .map(|s| s.parse())
                    .collect::<Result<Vec<_>, _>>().expect("failed to parse number");

                let res = sum(&mut numbers).expect("no items to compute sum");

                format!(
                    "resulting number is {}, it's magnitude is {}",
                    &res,
                    res.magnitude(),
                )
            }
            Part::Two => {
                let mut max_magnitude = None;

                for i in 0..self.numbers.len() {
                    for j in 0..self.numbers.len() {
                        let num_a = self.numbers[i].parse().expect("failed to parse number");
                        let num_b = self.numbers[j].parse().expect("failed to parse number");

                        let sum_node = sum(&mut [num_a, num_b]).unwrap();
                        let sum_magnitude = sum_node.magnitude();

                        match max_magnitude {
                            None => { max_magnitude.replace(sum_magnitude); }
                            Some(magnitude) if magnitude < sum_magnitude => {
                                max_magnitude.replace(sum_magnitude);
                            },
                            _ => {}
                        }
                    }
                }

                format!("max two-numbers sum magnitude is {}", max_magnitude.unwrap())
            },
        }
    }

    fn day_number() -> u32 {
        18
    }
}

impl FromStr for Solution {
    type Err = Box<dyn Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self {
            numbers: parse_raw_data(s)?,
        })
    }
}
