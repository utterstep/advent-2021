use std::{collections::BTreeMap, str::FromStr};

use displaydoc::Display;
use thiserror::Error;

#[derive(Debug, Clone, Copy)]
pub struct Rule {
    from: [char; 2],
    to: char,
}

/// Invalid rule format
#[derive(Debug, Display, Error)]
pub struct RuleParseError;

impl FromStr for Rule {
    type Err = RuleParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (from, to) = s.split_once(" -> ").ok_or(RuleParseError)?;
        let mut from_chars = from.chars();
        let mut to_chars = to.chars();

        Ok(Self {
            from: [
                from_chars.next().ok_or(RuleParseError)?,
                from_chars.next().ok_or(RuleParseError)?,
            ],
            to: to_chars.next().ok_or(RuleParseError)?,
        })
    }
}

#[derive(Debug)]
pub struct RuleSet {
    rules: BTreeMap<[char; 2], char>,
}

impl FromIterator<Rule> for RuleSet {
    fn from_iter<T: IntoIterator<Item = Rule>>(iter: T) -> Self {
        let rules = iter.into_iter().fold(BTreeMap::new(), |mut map, rule| {
            map.insert(rule.from, rule.to);

            map
        });

        Self { rules }
    }
}

impl RuleSet {
    pub fn apply(&self, pair: [char; 2]) -> [[char; 2]; 2] {
        let insertion = *self.rules.get(&pair).expect("unknown molecule");

        [[pair[0], insertion], [insertion, pair[1]]]
    }
}
