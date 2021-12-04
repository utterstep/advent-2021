use std::{error::Error, str::FromStr};

use advent_utils::{parse_raw_data, Part, Solver};

use command::Command;

mod command;

#[derive(Debug)]
pub struct Solution {
    commands: Vec<Command>,
}

impl FromStr for Solution {
    type Err = Box<dyn Error>;

    fn from_str(input_data: &str) -> Result<Self, Self::Err> {
        let commands = parse_raw_data(input_data)?;

        Ok(Self { commands })
    }
}

impl Solver for Solution {
    fn solve(&self, part: Part) -> String {
        match part {
            Part::One => {
                let mut x = 0;
                let mut depth = 0;

                for command in &self.commands {
                    match command {
                        Command::Down(n) => depth += n,
                        Command::Up(n) => depth -= n,
                        Command::Forward(n) => x += n,
                    }
                }

                format!("product of depth and x coord is {}", x * depth)
            }
            Part::Two => {
                let mut x = 0;
                let mut aim = 0;
                let mut depth = 0;

                for command in &self.commands {
                    match command {
                        Command::Down(n) => aim += n,
                        Command::Up(n) => aim -= n,
                        Command::Forward(n) => {
                            x += n;
                            depth += aim * n;
                        }
                    }
                }

                format!("product of depth and x coord is {}", x * depth)
            }
        }
    }

    fn day_number() -> u32 {
        2
    }
}
