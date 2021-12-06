use std::{error::Error, str::FromStr};

use advent_utils::{Part, Solver};

const PART_ONE_MOVES: usize = 80;
const PART_TWO_MOVES: usize = 256;

#[derive(Debug)]
pub struct Solution {
    lanternfish_timers: Vec<u64>,
}

impl FromStr for Solution {
    type Err = Box<dyn Error>;

    fn from_str(input_data: &str) -> Result<Self, Self::Err> {
        let lanternfish_timers = input_data
            .trim_end()
            .split(',')
            .map(str::parse)
            .collect::<Result<_, _>>()?;

        Ok(Self { lanternfish_timers })
    }
}

fn simulate_lanternfish_mating(timers: &[u64], moves: usize) -> [u64; 9] {
    let mut state = [0; 9];

    for &time in timers {
        state[time as usize] += 1;
    }

    for _ in 0..moves {
        let mut new_state = [0; 9];
        new_state[..8].copy_from_slice(&state[1..=8]);

        new_state[8] = state[0];
        new_state[6] += state[0];

        state = new_state;
    }

    state
}

impl Solver for Solution {
    fn solve(&self, part: Part) -> String {
        match part {
            Part::One => {
                let population_size: u64 =
                    simulate_lanternfish_mating(&self.lanternfish_timers, PART_ONE_MOVES)
                        .into_iter()
                        .sum();

                format!("population size after 80 moves is {}", population_size)
            }
            Part::Two => {
                let population_size: u64 =
                    simulate_lanternfish_mating(&self.lanternfish_timers, PART_TWO_MOVES)
                        .into_iter()
                        .sum();

                format!("population size after 256 moves is {}", population_size)
            }
        }
    }

    fn day_number() -> u32 {
        6
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one_example() {
        let input = [3, 4, 3, 1, 2];

        assert_eq!(
            simulate_lanternfish_mating(&input, PART_ONE_MOVES)
                .into_iter()
                .sum::<u64>(),
            5934,
        );
    }

    #[test]
    fn test_part_two_example() {
        let input = [3, 4, 3, 1, 2];

        assert_eq!(
            simulate_lanternfish_mating(&input, PART_TWO_MOVES)
                .into_iter()
                .sum::<u64>(),
            26984457539,
        );
    }
}
