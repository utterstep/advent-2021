use std::{error::Error, str::FromStr};

use advent_utils::{Part, Solver};
use rustc_hash::FxHashMap;

use minmax::minmax;
use rule::RuleSet;

mod minmax;
mod rule;

const N_STEPS_PART_ONE: usize = 10;
const N_STEPS_PART_TWO: usize = 40;

#[derive(Debug)]
pub struct Solution {
    first_char: char,
    last_char: char,
    pairs_counter: FxHashMap<[char; 2], u64>,
    rule_set: RuleSet,
}

impl FromStr for Solution {
    type Err = Box<dyn Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (initial, rules) = s.trim().split_once("\n\n").ok_or("invalid input format")?;

        let chars = initial.chars().collect::<Vec<_>>();
        let pairs_counter = chars.windows(2).map(|window| [window[0], window[1]]).fold(
            FxHashMap::default(),
            |mut map, pair| {
                *map.entry(pair).or_default() += 1;

                map
            },
        );
        let rule_set = rules.lines().map(str::parse).collect::<Result<_, _>>()?;

        Ok(Self {
            first_char: *chars.first().ok_or("empty initial string")?,
            last_char: *chars.last().ok_or("empty initial string")?,
            pairs_counter,
            rule_set,
        })
    }
}

impl Solver for Solution {
    fn solve(&self, part: Part) -> String {
        let n_steps = match part {
            Part::One => N_STEPS_PART_ONE,
            Part::Two => N_STEPS_PART_TWO,
        };

        let freq_map = self.perform_reactions(n_steps);
        let minmax = minmax(freq_map.values().copied()).expect("no items after reactions");

        format!(
            "maxfreq - minfreq after {} steps: {}",
            n_steps,
            minmax.1 - minmax.0,
        )
    }

    fn day_number() -> u32 {
        14
    }
}

impl Solution {
    fn perform_reactions(&self, n_steps: usize) -> FxHashMap<char, u64> {
        let mut old_pairs = self.pairs_counter.clone();
        let mut pairs = FxHashMap::default();

        for _ in 0..n_steps {
            pairs.reserve(old_pairs.len());

            old_pairs.into_iter().for_each(|(pair, count)| {
                let [pair_left, pair_right] = self.rule_set.apply(pair);

                *pairs.entry(pair_left).or_default() += count;
                *pairs.entry(pair_right).or_default() += count;
            });

            old_pairs = pairs;
            pairs = FxHashMap::default();
        }

        let mut char_counter = old_pairs
            .into_iter()
            .map(|(pair, count)| [(pair[0], count), (pair[1], count)])
            .flatten()
            .fold(FxHashMap::default(), |mut map, (char, count)| {
                *map.entry(char).or_default() += count;

                map
            });

        // all characters are counted twice (as left in pair and as right in pair)...
        char_counter.values_mut().for_each(|count| *count /= 2);

        // ...except first and last ones (there are only one pair for each one of them)
        char_counter
            .entry(self.first_char)
            .and_modify(|count| *count += 1);
        char_counter
            .entry(self.last_char)
            .and_modify(|count| *count += 1);

        char_counter
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example() {
        let solution: Solution = include_str!("../example.txt")
            .parse()
            .expect("failed to parse input");

        // part one – ten steps
        let freq_map = solution.perform_reactions(N_STEPS_PART_ONE);
        assert_eq!(freq_map.get(&'B'), Some(&1749));
        assert_eq!(freq_map.get(&'C'), Some(&298));
        assert_eq!(freq_map.get(&'H'), Some(&161));
        assert_eq!(freq_map.get(&'N'), Some(&865));

        // part two – forty steps
        let freq_map = solution.perform_reactions(N_STEPS_PART_TWO);
        assert_eq!(freq_map.get(&'B'), Some(&2192039569602));
        assert_eq!(freq_map.get(&'H'), Some(&3849876073));
    }
}
