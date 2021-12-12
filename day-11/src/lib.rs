use std::{collections::BTreeSet, error::Error, iter::once, str::FromStr};

use advent_utils::{Part, Solver};

#[derive(Debug)]
pub struct Solution {
    octopuses: Vec<Vec<u8>>,
}

const N_STEPS: usize = 100;

impl FromStr for Solution {
    type Err = Box<dyn Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut octopuses: Vec<Vec<u8>> = s
            .trim_end()
            .split('\n')
            .map(|line| {
                once(u8::MAX)
                    .chain(line.bytes().map(|b| b - b'0'))
                    .chain(once(u8::MAX))
                    .collect()
            })
            .collect();

        let line_size = octopuses[0].len();

        octopuses.insert(0, vec![u8::MAX; line_size]);
        octopuses.push(vec![u8::MAX; line_size]);

        Ok(Self { octopuses })
    }
}

fn step(octopuses: &mut [Vec<u8>]) -> usize {
    let mut process_flashed = vec![];
    let mut flashed = BTreeSet::new();

    for y in 1..(octopuses.len() - 1) {
        for x in 1..(octopuses[y].len() - 1) {
            octopuses[y][x] += 1;

            if octopuses[y][x] > 9 {
                flashed.insert((x, y));

                process_flashed.push((x, y));
            }
        }
    }

    while let Some((x, y)) = process_flashed.pop() {
        for (x_n, y_n) in [
            (x - 1, y - 1),
            (x, y - 1),
            (x + 1, y - 1),
            (x - 1, y),
            (x + 1, y),
            (x - 1, y + 1),
            (x, y + 1),
            (x + 1, y + 1),
        ] {
            if octopuses[y_n][x_n] == u8::MAX {
                continue;
            }

            octopuses[y_n][x_n] += 1;

            if octopuses[y_n][x_n] > 9 && flashed.insert((x_n, y_n)) {
                process_flashed.push((x_n, y_n));
            }
        }
    }

    for &(x, y) in flashed.iter() {
        octopuses[y][x] = 0;
    }

    flashed.len()
}

impl Solver for Solution {
    fn solve(&self, part: Part) -> String {
        let mut octopuses = self.octopuses.clone();

        match part {
            Part::One => {
                let total_flashes = (0..N_STEPS)
                    .map(|_| step(&mut octopuses))
                    .sum::<usize>();

                format!("there were total {} flashes", total_flashes)
            },
            Part::Two => {
                let total_octopuses = (octopuses.len() - 2) * (octopuses[0].len() - 2);

                match (1..).find(|_| step(&mut octopuses) == total_octopuses) {
                    Some(step_no) => format!(
                        "all octopuses will be synchronized after {} steps",
                        step_no
                    ),
                    None => "octopuses failed to synchronize :(".to_owned()
                }
            }
        }
    }

    fn day_number() -> u32 {
        11
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    #[test]
    fn test_small_example() {
        let solution: Solution = indoc!("
            11111
            19991
            19191
            19991
            11111"
        ).parse().unwrap();

        let mut octopuses = solution.octopuses.clone();

        assert_eq!(step(&mut octopuses), 9);
        assert_eq!(step(&mut octopuses), 0);
    }

    #[test]
    fn test_larger_example() {
        let solution: Solution = indoc!("
            5483143223
            2745854711
            5264556173
            6141336146
            6357385478
            4167524645
            2176841721
            6882881134
            4846848554
            5283751526"
        ).parse().unwrap();

        assert_eq!(
            solution.solve(Part::One),
            "there were total 1656 flashes"
        );

        assert_eq!(
            solution.solve(Part::Two),
            "all octopuses will be synchronized after 195 steps"
        );
    }
}