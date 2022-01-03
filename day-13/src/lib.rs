use std::{collections::BTreeSet, error::Error, str::FromStr};

use advent_utils::{parse_raw_data, Part, Solver};

use point::Point;
use rule::Rule;

mod point;
mod rule;

#[derive(Debug)]
pub struct Solution {
    points: BTreeSet<Point>,
    rules: Vec<Rule>,
}

impl FromStr for Solution {
    type Err = Box<dyn Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (points, rules) = s.split_once("\n\n").expect("invalid input format");

        let points = parse_raw_data(points)?;
        let rules = parse_raw_data(rules)?;

        Ok(Self {
            points: BTreeSet::from_iter(points.into_iter()),
            rules,
        })
    }
}

fn format_points(points: &BTreeSet<Point>) -> Option<String> {
    let up_right = points.iter().copied().reduce(|mut up_right, point| {
        if point.x > up_right.x {
            up_right.x = point.x;
        }

        if point.y > up_right.y {
            up_right.y = point.y;
        }

        up_right
    })?;

    let down_left = points.iter().copied().reduce(|mut down_left, point| {
        if point.x < down_left.x {
            down_left.x = point.x;
        }

        if point.y < down_left.y {
            down_left.y = point.y;
        }

        down_left
    })?;

    let x_span = up_right.x - down_left.x + 1;
    let y_span = up_right.y - down_left.y + 1;

    let mut canvas = vec![vec![b' '; x_span as usize]; y_span as usize];

    for point in points {
        let x = usize::try_from(point.x - down_left.x).ok()?;
        let y = usize::try_from(point.y - down_left.y).ok()?;

        canvas[y as usize][x as usize] = b'#';
    }

    let mut result = String::with_capacity(((x_span + 1) * y_span) as usize);

    for line in canvas {
        // SAFETY: just built this string manually, using only b'.' and b'#'
        result.push_str(unsafe { std::str::from_utf8_unchecked(&line) });
        result.push('\n');
    }

    Some(result)
}

impl Solver for Solution {
    fn solve(&self, part: Part) -> String {
        match part {
            Part::One => {
                let points_after_first_fold = self.rules[0].perform(&self.points);

                format!(
                    "there will be {} points visible after first folds",
                    points_after_first_fold.len()
                )
            }
            Part::Two => {
                let points_result = self
                    .rules
                    .iter()
                    .fold(self.points.clone(), |points, rule| rule.perform(&points));

                format_points(&points_result).expect("failed to format points")
            }
        }
    }

    fn day_number() -> u32 {
        13
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example() {
        let solution: Solution = include_str!("../example.txt").parse().unwrap();

        let points_after_first_fold = solution.rules[0].perform(&solution.points);
        assert_eq!(points_after_first_fold.len(), 17);

        let points_after_second_fold = solution.rules[1].perform(&points_after_first_fold);
        assert_eq!(points_after_second_fold.len(), 16);
    }
}
