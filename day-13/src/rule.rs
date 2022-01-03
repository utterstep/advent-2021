use std::{collections::BTreeSet, num::ParseIntError, str::FromStr};

use displaydoc::Display;
use thiserror::Error;

use crate::point::Point;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Rule {
    FoldX(i32),
    FoldY(i32),
}

#[derive(Debug, Display, Error)]
/// Failed to parse line
pub enum ParsePointError {
    /// Invalid data format
    InvalidFormat,
    /// Failed to parse coordinate: {0}
    ParseIntError(#[from] ParseIntError),
}

impl FromStr for Rule {
    type Err = ParsePointError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.split_once('=') {
            Some(("fold along x", num)) => Ok(Self::FoldX(num.parse()?)),
            Some(("fold along y", num)) => Ok(Self::FoldY(num.parse()?)),
            _ => Err(ParsePointError::InvalidFormat),
        }
    }
}

impl Rule {
    pub fn perform(self, points: &BTreeSet<Point>) -> BTreeSet<Point> {
        points
            .iter()
            .map(|&point| match self {
                Self::FoldX(x_corr) => {
                    if point.x < x_corr {
                        point
                    } else {
                        (2 * x_corr - point.x, point.y).into()
                    }
                }
                Self::FoldY(y_corr) => {
                    if point.y < y_corr {
                        point
                    } else {
                        (point.x, 2 * y_corr - point.y).into()
                    }
                }
            })
            .collect()
    }
}
