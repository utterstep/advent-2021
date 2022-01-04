#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Operator {
    Sum = 0,
    Product = 1,
    Minimum = 2,
    Maximum = 3,
    GreaterThan = 5,
    LessThan = 6,
    EqualTo = 7,
}

use super::ParsePacketError;

impl Operator {
    pub(super) fn operands_count(&self) -> Option<usize> {
        match *self {
            Self::GreaterThan | Self::LessThan | Self::EqualTo => Some(2),
            _ => None,
        }
    }
}

impl TryFrom<u8> for Operator {
    type Error = ParsePacketError;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            x if x == Self::Sum as u8 => Ok(Self::Sum),
            x if x == Self::Product as u8 => Ok(Self::Product),
            x if x == Self::Minimum as u8 => Ok(Self::Minimum),
            x if x == Self::Maximum as u8 => Ok(Self::Maximum),
            x if x == Self::GreaterThan as u8 => Ok(Self::GreaterThan),
            x if x == Self::LessThan as u8 => Ok(Self::LessThan),
            x if x == Self::EqualTo as u8 => Ok(Self::EqualTo),
            _ => Err(ParsePacketError::InvalidPacketFormat),
        }
    }
}
