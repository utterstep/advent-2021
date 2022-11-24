use std::{num::ParseIntError, str::FromStr};

use displaydoc::Display;
use thiserror::Error;

#[derive(Debug, PartialEq, Eq)]
pub struct LineSegment {
    start: (i32, i32),
    end: (i32, i32),
}

#[derive(Debug, Display, Error)]
/// Failed to parse line
pub enum ParseLineError {
    /// Invalid data format
    InvalidFormat,
    /// Failed to parse coordinate: {0}
    ParseIntError(#[from] ParseIntError),
}

impl FromStr for LineSegment {
    type Err = ParseLineError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (start, end) = s.split_once(" -> ").ok_or(ParseLineError::InvalidFormat)?;

        let (start_x, start_y) = start.split_once(',').ok_or(ParseLineError::InvalidFormat)?;
        let (end_x, end_y) = end.split_once(',').ok_or(ParseLineError::InvalidFormat)?;

        Ok(Self {
            start: (start_x.parse()?, start_y.parse()?),
            end: (end_x.parse()?, end_y.parse()?),
        })
    }
}

impl LineSegment {
    pub fn is_horizontal(&self) -> bool {
        self.start.1 == self.end.1
    }

    pub fn is_vertical(&self) -> bool {
        self.start.0 == self.end.0
    }

    pub fn is_diagonal(&self) -> bool {
        let a = self.end.0 - self.start.0;
        let b = self.end.1 - self.start.1;

        a.abs() == b.abs()
    }

    pub fn to_int_points(&self) -> Vec<(i32, i32)> {
        if self.is_horizontal() {
            let y = self.start.1;
            let x_start = self.start.0;
            let x_end = self.end.0;

            let (x_start, x_end) = (x_start.min(x_end), x_start.max(x_end));

            (x_start..=x_end).map(|x| (x, y)).collect()
        } else if self.is_vertical() {
            let x = self.start.0;
            let y_start = self.start.1;
            let y_end = self.end.1;

            let (y_start, y_end) = (y_start.min(y_end), y_start.max(y_end));

            (y_start..=y_end).map(|y| (x, y)).collect()
        } else if self.is_diagonal() {
            let n_steps = (self.start.0 - self.end.0).abs() as i32;

            let x_diff = (self.end.0 - self.start.0) as i32 / n_steps;
            let y_diff = (self.end.1 - self.start.1) as i32 / n_steps;

            let mut x = self.start.0 as i32;
            let mut y = self.start.1 as i32;

            let mut points = Vec::with_capacity(n_steps as usize);

            for _ in 0..=n_steps {
                points.push((x, y));
                x += x_diff;
                y += y_diff;
            }

            points
        } else {
            unreachable!()
        }
    }
}
