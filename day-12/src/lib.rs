use std::{collections::BTreeMap, error::Error, str::FromStr};

use advent_utils::{Part, Solver};
use smallvec::SmallVec;
use string_interner::{DefaultSymbol, StringInterner};

#[derive(Debug)]
pub struct Solution {
    /// there are a lot of small string, resulting in trivial implementation time
    /// being dwarfed by allocation time, so I use string interning here to get rid
    /// of most of allocations (reduces runtime ~threefold)
    interner: StringInterner,
    /// Map, containing the caves, accessible from the given cave
    caves_map: BTreeMap<Cave, SmallVec<[Cave; 32]>>,
}

const START: &str = "start";
const END: &str = "end";

impl FromStr for Solution {
    type Err = Box<dyn Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut interner = StringInterner::new();
        let mut cave_map: BTreeMap<Cave, SmallVec<[Cave; 32]>> = BTreeMap::new();

        for line in s.lines() {
            if let Some((from, to)) = line.split_once('-') {
                let from_cave = Cave::from_str(from, &mut interner);
                let to_cave = Cave::from_str(to, &mut interner);

                cave_map.entry(from_cave).or_default().push(to_cave);
                cave_map.entry(to_cave).or_default().push(from_cave);
            }
        }

        Ok(Self {
            interner,
            caves_map: cave_map,
        })
    }
}

impl Solution {
    fn _paths(
        &self,
        from: Cave,
        to: Cave,
        mut visited_small: SmallVec<[Cave; 12]>,
        mut visited_twice: Option<Cave>,
    ) -> usize {
        if from == to {
            return 1;
        }

        #[allow(clippy::never_loop)]
        // the loop here allows to do early break. A bit cryptic, I agree.
        // Will think about it later
        loop {
            if from.is_small {
                if !visited_small.contains(&from) {
                    visited_small.push(from);

                    break;
                }

                if visited_twice.is_none() && self.interner.resolve(from.symbol) != Some(START) {
                    visited_twice = Some(from);
                } else {
                    return 0;
                }
            }

            break;
        }

        if let Some(out_caves) = self.caves_map.get(&from) {
            return out_caves
                .iter()
                .map(|&out_cave| self._paths(out_cave, to, visited_small.clone(), visited_twice))
                .sum();
        }

        0
    }

    fn paths(&self, from: &str, to: &str, allow_twice: bool) -> Option<usize> {
        let from_cave = Cave::try_from_str(from, &self.interner)?;
        let to_cave = Cave::try_from_str(to, &self.interner)?;

        // hack to avoid creating one more function argument
        let visited_twice = (!allow_twice).then_some(to_cave);

        Some(self._paths(
            from_cave,
            to_cave,
            SmallVec::with_capacity(12),
            visited_twice,
        ))
    }
}

impl Solver for Solution {
    fn solve(&self, part: Part) -> String {
        match part {
            Part::One => {
                format!(
                    "there are {} paths from `start` to `end`",
                    self.paths(START, END, false)
                        .expect("failed to compute paths")
                )
            }
            Part::Two => {
                format!(
                    "there are {} paths from `start` to `end`, if we can visit one small cave twice",
                    self.paths(START, END, true).expect("failed to compute paths")
                )
            }
        }
    }

    fn day_number() -> u32 {
        12
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
struct Cave {
    symbol: DefaultSymbol,
    is_small: bool,
}

impl Cave {
    fn from_str(s: &str, interner: &mut StringInterner) -> Self {
        let symbol = interner.get_or_intern(s);

        Self {
            symbol,
            is_small: s.chars().all(|c| c.is_ascii_lowercase()),
        }
    }

    fn try_from_str(s: &str, interner: &StringInterner) -> Option<Self> {
        let symbol = interner.get(&s)?;

        Some(Self {
            symbol,
            is_small: s.chars().all(|c| c.is_ascii_lowercase()),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_examples_once() {
        let solution: Solution = include_str!("../example-10-36.txt").parse().unwrap();
        assert_eq!(solution.paths(START, END, false), Some(10));

        let solution: Solution = include_str!("../example-19-103.txt").parse().unwrap();
        assert_eq!(solution.paths(START, END, false), Some(19));

        let solution: Solution = include_str!("../example-226-3509.txt").parse().unwrap();
        assert_eq!(solution.paths(START, END, false), Some(226));
    }

    #[test]
    fn test_examples_twice() {
        let solution: Solution = include_str!("../example-10-36.txt").parse().unwrap();
        assert_eq!(solution.paths(START, END, true), Some(36));

        let solution: Solution = include_str!("../example-19-103.txt").parse().unwrap();
        assert_eq!(solution.paths(START, END, true), Some(103));

        let solution: Solution = include_str!("../example-226-3509.txt").parse().unwrap();
        assert_eq!(solution.paths(START, END, true), Some(3509));
    }
}
