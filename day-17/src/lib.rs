use std::{error::Error, ops::RangeInclusive, str::FromStr};

use advent_utils::{Part, Solver};

use math::{compute_hit, compute_x_velocity, sum_up_to_n, HorizontalBoundary};

mod math;

#[derive(Debug)]
pub struct Solution {
    x_range: RangeInclusive<i64>,
    y_range: RangeInclusive<i64>,
}

const PREFIX: &str = "target area: ";
const UNKNOWN_FORMAT_ERR: &str = "unknown format";

impl Solver for Solution {
    fn solve(&self, part: Part) -> String {
        let max_y_vel = self.y_range.start().abs().max(self.y_range.end().abs()) - 1;

        match part {
            Part::One => format!(
                "max speed is {}, max height is {}",
                max_y_vel,
                sum_up_to_n(max_y_vel),
            ),
            Part::Two => {
                let min_y_vel = *self.y_range.start().min(self.y_range.end());

                let min_x_vel = compute_x_velocity(&self.x_range, HorizontalBoundary::Left)
                    .expect("failed to compute x velocity");
                let max_x_vel = *self.x_range.end();

                cfg_if::cfg_if! {
                    if #[cfg(debug_assertions)] {
                        let mut vels = Vec::new();
                    } else {
                        let mut count = 0;
                    }
                }

                for x_vel in min_x_vel..=max_x_vel {
                    for y_vel in min_y_vel..=max_y_vel {
                        if let Some(_hit) = compute_hit(&self.x_range, &self.y_range, x_vel, y_vel)
                        {
                            cfg_if::cfg_if! {
                                if #[cfg(debug_assertions)] {
                                    vels.push((x_vel, y_vel));
                                } else {
                                    count += 1;
                                }
                            }
                        }
                    }
                }

                cfg_if::cfg_if! {
                    if #[cfg(debug_assertions)] {
                        dbg!(&vels);

                        let count = vels.len();
                    }
                };

                format!("there are {} possible initial velocities", count)
            }
        }
    }

    fn day_number() -> u32 {
        17
    }
}

impl FromStr for Solution {
    type Err = Box<dyn Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s = s.trim_end();

        macro_rules! pattern_or_error {
            ($val:expr, $pat:pat => $success:expr) => {
                match $val {
                    $pat => $success,
                    _ => return Err(UNKNOWN_FORMAT_ERR.into()),
                }
            };
        }

        let coords = pattern_or_error!(&s[..PREFIX.len()], PREFIX => &s[PREFIX.len()..]);

        let (x_range, y_range) = pattern_or_error!(coords.split_once(", "), Some((x, y)) => (x, y));

        let x_range = pattern_or_error!(x_range.split_once('='), Some(("x", value)) => value);
        let y_range = pattern_or_error!(y_range.split_once('='), Some(("y", value)) => value);

        let (x_start, x_end) =
            pattern_or_error!(x_range.split_once(".."), Some((x_start, x_end)) => (x_start, x_end));
        let (y_start, y_end) =
            pattern_or_error!(y_range.split_once(".."), Some((y_start, y_end)) => (y_start, y_end));

        let x_start = x_start.parse()?;
        let x_end = x_end.parse()?;

        let y_start = y_start.parse()?;
        let y_end = y_end.parse()?;

        Ok(Self {
            x_range: x_start..=x_end,
            y_range: y_start..=y_end,
        })
    }
}
