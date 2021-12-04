use std::{num::ParseIntError, str::FromStr};

use displaydoc::Display;
use thiserror::Error;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum Command {
    Up(i32),
    Down(i32),
    Forward(i32),
}

/// Failed to parse command
#[derive(Debug, Display, Error)]
pub enum ParseCommandError {
    /// Invalid command format
    InvalidFormat,
    /// Unknown command
    UnknownCommand,
    /// Failed to parse int: {0}
    IntParseError(#[from] ParseIntError),
}

impl FromStr for Command {
    type Err = ParseCommandError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut splitted = s.split_ascii_whitespace();
        let command = splitted.next().ok_or(ParseCommandError::InvalidFormat)?;
        let value = splitted.next().ok_or(ParseCommandError::InvalidFormat)?;

        match command {
            "forward" => Ok(Self::Forward(value.parse()?)),
            "up" => Ok(Self::Up(value.parse()?)),
            "down" => Ok(Self::Down(value.parse()?)),
            _ => Err(ParseCommandError::UnknownCommand),
        }
    }
}
