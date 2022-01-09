use std::{num::ParseIntError, str::FromStr};

use displaydoc::Display;
use thiserror::Error;

use crate::node::{Node, NodeValue};

#[derive(Debug, Display, Error)]
/// Error while parsing node
pub enum ParseNodeError {
    /// Invalid int literal: {0}
    ParseIntError(#[from] ParseIntError),
    /// Invalid node format
    InvalidFormat,
}

type ParseResult<'a, T> = Result<(T, &'a str), ParseNodeError>;

fn find_left_right(s: &str) -> Option<(&str, &str)> {
    let mut braces = 0;

    s.split_once(|c| {
        if c == '[' {
            braces += 1;
        } else if c == ']' {
            braces -= 1;
        }

        braces == 1 && c == ','
    })
}

fn parse_node(s: &str) -> ParseResult<Node> {
    let rest = s;

    if !rest.starts_with('[') {
        return Err(ParseNodeError::InvalidFormat);
    }

    let (left, right) = find_left_right(rest).ok_or(ParseNodeError::InvalidFormat)?;

    let left = left
        .strip_prefix('[')
        .ok_or(ParseNodeError::InvalidFormat)?;
    let right = right
        .strip_suffix(']')
        .ok_or(ParseNodeError::InvalidFormat)?;

    let (left, _rest) = parse_node_value(left)?;
    assert!(_rest.is_empty());
    let (right, rest) = parse_node_value(right)?;

    Ok((Node::new(left, right), rest))
}

fn parse_node_value(s: &str) -> ParseResult<NodeValue> {
    if s.starts_with('[') {
        let (node, rest) = parse_node(s)?;
        Ok((NodeValue::Node(node), rest))
    } else {
        let (literal, rest) = s
            .split_once(&[',', ']'][..])
            .unwrap_or_else(|| (&s[..s.len()], &s[s.len()..]));

        Ok((NodeValue::Literal(literal.parse()?), rest))
    }
}

impl FromStr for Node {
    type Err = ParseNodeError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (node, rest) = parse_node(s)?;

        if rest.is_empty() {
            Ok(node)
        } else {
            Err(ParseNodeError::InvalidFormat)
        }
    }
}
