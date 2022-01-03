use std::{num::ParseIntError, str::FromStr};

use displaydoc::Display;
use thiserror::Error;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Point {
    pub(crate) x: i32,
    pub(crate) y: i32,
}

#[derive(Debug, Display, Error)]
/// Failed to parse line
pub enum ParsePointError {
    /// Invalid data format
    InvalidFormat,
    /// Failed to parse coordinate: {0}
    ParseIntError(#[from] ParseIntError),
}

impl FromStr for Point {
    type Err = ParsePointError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (x, y) = s.split_once(',').ok_or(ParsePointError::InvalidFormat)?;

        Ok(Self {
            x: x.parse()?,
            y: y.parse()?,
        })
    }
}

impl From<(i32, i32)> for Point {
    fn from((x, y): (i32, i32)) -> Self {
        Self { x, y }
    }
}
