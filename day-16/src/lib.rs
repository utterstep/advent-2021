use std::{error::Error, str::FromStr};

use advent_utils::{Part, Solver};

use packet::{
    visitor::{ExpressionEvalVisitor, VersionSumVisitor, Visitor},
    Packet,
};

mod packet;

#[derive(Debug)]
pub struct Solution {
    packet_tree: Packet,
}

impl Solver for Solution {
    fn solve(&self, part: Part) -> String {
        match part {
            Part::One => {
                let mut visitor = VersionSumVisitor::default();

                format!(
                    "sum of versions is {}",
                    visitor.visit_packet(&self.packet_tree)
                )
            }
            Part::Two => {
                let mut visitor = ExpressionEvalVisitor::default();

                format!("expr value is {}", visitor.visit_packet(&self.packet_tree))
            }
        }
    }

    fn day_number() -> u32 {
        16
    }
}

impl FromStr for Solution {
    type Err = Box<dyn Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self {
            packet_tree: s.trim_end().parse()?,
        })
    }
}
