use std::{
    borrow::Cow,
    cmp::Reverse,
    collections::BinaryHeap,
    error::Error,
    str::FromStr,
};

use advent_utils::{Part, Solver};
use rustc_hash::FxHashMap;

#[derive(Debug)]
pub struct Solution {
    risk_factors: Vec<Vec<u8>>,
}

impl Solver for Solution {
    fn solve(&self, part: Part) -> String {
        let risk_factors = match part {
            Part::One => Cow::Borrowed(&self.risk_factors),
            Part::Two => Cow::Owned(repeat_map(&self.risk_factors, 5)),
        };

        let end = (
            risk_factors.len() - 1,
            risk_factors[risk_factors.len() - 1].len() - 1,
        );

        format!(
            "safest path from (0, 0) to {:?} has total risk factor of: {}",
            end,
            a_star((0, 0), end, &risk_factors).expect("failed to find path"),
        )
    }

    fn day_number() -> u32 {
        15
    }
}

fn repeat_map(map: &[Vec<u8>], n_times: usize) -> Vec<Vec<u8>> {
    let resized_horizontally = map
        .iter()
        .map(|line| {
            let mut line = line.clone();
            let width = line.len();

            line.reserve_exact(width * 5);

            for i in 0..n_times - 1 {
                for j in 0..width {
                    line.push(match line[i * width + j] {
                        9 => 1,
                        n => n + 1,
                    })
                }
            }

            line
        })
        .collect::<Vec<_>>();

    let mut new_map = resized_horizontally;
    let height = new_map.len();
    new_map.reserve_exact(height * 5);

    for i in 0..n_times - 1 {
        for j in 0..height {
            let mut line = new_map[i * height + j].clone();

            for item in line.iter_mut() {
                *item = match *item {
                    9 => 1,
                    n => n + 1,
                }
            }

            new_map.push(line);
        }
    }

    new_map
}

fn a_star(from: (usize, usize), to: (usize, usize), map: &[Vec<u8>]) -> Option<u64> {
    fn price(from: (usize, usize), to: (usize, usize)) -> u64 {
        let x = from.0.max(to.0) - from.0.min(to.0);
        let y = from.1.max(to.1) - from.1.min(to.1);

        (x + y) as u64
    }

    fn neighbours((x, y): (usize, usize), map: &[Vec<u8>]) -> impl Iterator<Item = (usize, usize)> {
        let height = map.len();
        let width = map[y].len();

        [
            x.checked_sub(1).map(|x| (x, y)),
            y.checked_sub(1).map(|y| (x, y)),
            (x < (width - 1)).then(|| (x + 1, y)),
            (y < (height - 1)).then(|| (x, y + 1)),
        ]
        .into_iter()
        .flatten()
    }

    let mut path_prices = FxHashMap::default();
    let mut to_visit = BinaryHeap::new();

    path_prices.insert(from, 0);
    to_visit.push((Reverse(0), from));

    while let Some((_cost, current)) = to_visit.pop() {
        if current == to {
            return path_prices.get(&current).copied();
        }

        for neighbour in neighbours(current, map) {
            let neighbour_cost_from_current =
                *path_prices.get(&current)? + map[neighbour.1][neighbour.0] as u64;

            match path_prices.get(&neighbour) {
                Some(_neighbour_cost) if *_neighbour_cost <= neighbour_cost_from_current => {}
                _ => {
                    path_prices.insert(neighbour, neighbour_cost_from_current);
                    to_visit.push((
                        Reverse(neighbour_cost_from_current + price(neighbour, to)),
                        neighbour,
                    ));
                }
            }
        }
    }

    None
}

impl FromStr for Solution {
    type Err = Box<dyn Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let risk_factors: Vec<Vec<u8>> = s
            .trim_end()
            .split('\n')
            .map(|line| line.bytes().map(|b| b - b'0').collect())
            .collect();

        Ok(Self { risk_factors })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example() {
        let solution: Solution = include_str!("../example.txt")
            .parse()
            .expect("failed to parse solution");

        assert_eq!(a_star((0, 0), (9, 9), &solution.risk_factors), Some(40));
    }

    #[test]
    fn test_example_part_two() {
        let solution: Solution = include_str!("../example.txt")
            .parse()
            .expect("failed to parse solution");

        let risk_factors_part_two = repeat_map(&solution.risk_factors, 5);

        assert_eq!(a_star((0, 0), (9, 9), &risk_factors_part_two), Some(40));
        assert_eq!(a_star((0, 0), (49, 49), &risk_factors_part_two), Some(315));
    }
}
