use std::{collections::BTreeSet, error::Error, iter::once, str::FromStr};

use advent_utils::{Part, Solver};

#[derive(Debug)]
pub struct Solution {
    heights: Vec<Vec<u8>>,
}

impl FromStr for Solution {
    type Err = Box<dyn Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut heights: Vec<Vec<u8>> = s
            .lines()
            .map(|line| {
                once(u8::MAX)
                    .chain(line.bytes().map(|b| b - b'0'))
                    .chain(once(u8::MAX))
                    .collect()
            })
            .collect();

        let line_size = heights[0].len();

        heights.insert(0, vec![u8::MAX; line_size]);
        heights.push(vec![u8::MAX; line_size]);

        Ok(Self { heights })
    }
}

fn find_basin_size(heights: &[Vec<u8>], x: usize, y: usize) -> usize {
    let mut to_visit_queue = vec![(x, y, heights[y][x])];
    let mut set = BTreeSet::new();
    set.insert((x, y));

    while let Some((x, y, current)) = to_visit_queue.pop() {
        for coord in [(x + 1, y), (x - 1, y), (x, y + 1), (x, y - 1)] {
            let value = heights[coord.1][coord.0];

            if value > current && set.insert(coord) && value < 9 {
                to_visit_queue.push((coord.0, coord.1, value))
            }
        }
    }

    set.len()
}

impl Solver for Solution {
    fn solve(&self, part: Part) -> String {
        let mut low_points = vec![];

        for (y, rows_window) in self.heights.windows(3).enumerate() {
            for (x, row) in rows_window[1].windows(3).enumerate() {
                if row[1] < row[0]
                    && row[1] < row[2]
                    && row[1] < rows_window[0][x + 1]
                    && row[1] < rows_window[2][x + 1]
                {
                    low_points.push((x + 1, y + 1));
                }
            }
        }

        match part {
            Part::One => {
                format!(
                    "sum of risk values is {}",
                    low_points
                        .iter()
                        .map(|&(x, y)| self.heights[y][x] as u32)
                        .sum::<u32>()
                        + low_points.len() as u32
                )
            }
            Part::Two => {
                let mut basin_sizes = low_points
                    .iter()
                    .map(|&(x, y)| find_basin_size(&self.heights, x, y))
                    .collect::<Vec<_>>();

                basin_sizes.sort_unstable();

                format!(
                    "product of basin sizes is {}",
                    basin_sizes.into_iter().rev().take(3).product::<usize>()
                )
            }
        }
    }

    fn day_number() -> u32 {
        9
    }
}
