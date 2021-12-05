use std::{
    error::Error,
    str::FromStr,
};

use advent_utils::{parse_raw_data, Part, Solver};
use rustc_hash::FxHashMap;

use line::LineSegment;

mod line;

#[derive(Debug)]
pub struct Solution {
    lines: Vec<LineSegment>,
}

impl FromStr for Solution {
    type Err = Box<dyn Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let lines = parse_raw_data(s)?;

        Ok(Self { lines })
    }
}

impl Solver for Solution {
    fn solve(&self, part: Part) -> String {
        match part {
            Part::One => {
                let lines = self
                    .lines
                    .iter()
                    .filter(|&line| line.is_horizontal() || line.is_vertical());

                let mut points: FxHashMap<_, u32> = FxHashMap::default();

                for line in lines {
                    for point in line.to_int_points() {
                        *points.entry(point).or_default() += 1;
                    }
                }

                format!("there is {} points with overlap", points.values().filter(|&&v| v > 1).count())
            }
            Part::Two => {
                let mut points: FxHashMap<_, u32> = FxHashMap::default();

                for line in &self.lines {
                    for point in line.to_int_points() {
                        *points.entry(point).or_default() += 1;
                    }
                }

                format!("there is {} points with overlap", points.values().filter(|&&v| v > 1).count())
            }
        }
    }

    fn day_number() -> u32 {
        5
    }
}
