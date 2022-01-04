use std::{
    borrow::Cow,
    num::ParseIntError,
    str::{FromStr, Utf8Error},
};

use bitvec::{field::BitField, order::Msb0, slice::BitSlice, view::BitView};
use displaydoc::Display;
use thiserror::Error;

use operator::Operator;

pub mod operator;
pub mod visitor;

type ContainerType = u64;

#[derive(Debug)]
pub struct Packet {
    version: u8,
    payload: PacketPayload,
}

#[derive(Debug)]
pub enum PacketPayload {
    Literal(u64),
    Expression {
        operator: Operator,
        operands: Vec<Packet>,
    },
}

#[derive(Debug, Display, Error)]
/// An error which can occure during package parsing
pub enum ParsePacketError {
    /// Failed to parse int from hex representation
    ParseIntError(#[from] ParseIntError),
    /// Invalid packet format
    InvalidPacketFormat,
    /// String wasn't ASCII
    InvalidString(#[from] Utf8Error),
}

impl FromStr for Packet {
    type Err = ParsePacketError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        const CHUNK_SIZE: usize = (ContainerType::BITS / 4) as usize;

        let hex_numbers = s
            .as_bytes()
            .chunks(CHUNK_SIZE)
            .map(|chunk| {
                let chunk = if chunk.len() < CHUNK_SIZE {
                    let mut new_chunk = vec![b'0'; CHUNK_SIZE];
                    new_chunk[..chunk.len()].copy_from_slice(chunk);
                    Cow::Owned(new_chunk)
                } else {
                    Cow::Borrowed(chunk)
                };
                Ok(ContainerType::from_str_radix(
                    std::str::from_utf8(&chunk)?,
                    16,
                )?)
            })
            .collect::<Result<Vec<_>, ParsePacketError>>()?;

        let bits = hex_numbers.view_bits::<Msb0>();

        Ok(parse_packet(bits)?.0)
    }
}

type Bits = BitSlice<Msb0, ContainerType>;
type ParseResult<'a, T> = Result<(T, &'a Bits), ParsePacketError>;

fn parse_packet(value: &Bits) -> ParseResult<Packet> {
    let (version, rest) = value.split_at(3);
    let (payload, rest) = parse_packet_payload(rest)?;

    Ok((
        Packet {
            version: version.load_be::<u8>(),
            payload,
        },
        rest,
    ))
}

fn parse_packet_payload(value: &Bits) -> ParseResult<PacketPayload> {
    let (type_id, rest) = value.split_at(3);

    match type_id.load_be::<u8>() {
        4 => {
            let (literal, rest) = parse_literal(rest)?;
            Ok((PacketPayload::Literal(literal), rest))
        }
        type_id => {
            let (length_type, rest) = rest.split_at(1);

            let operator: Operator = type_id.try_into()?;
            let (operands, rest) = match length_type.load_be::<u8>() {
                0 => parse_packets_bit_length(rest)?,
                1 => parse_packets_n_items(rest)?,
                _ => unreachable!(),
            };

            if let Some(required_operands_count) = operator.operands_count() {
                if operands.len() != required_operands_count {
                    return Err(ParsePacketError::InvalidPacketFormat);
                }
            }

            Ok((PacketPayload::Expression { operator, operands }, rest))
        }
    }
}

fn parse_packets_bit_length(value: &Bits) -> ParseResult<Vec<Packet>> {
    let (length, rest) = value.split_at(15);

    let length = length.load_be::<usize>();
    let (mut packets_data, rest) = rest.split_at(length);
    let mut packets = vec![];

    while !packets_data.is_empty() {
        let (packet, packets_data_rest) = parse_packet(packets_data)?;

        packets.push(packet);
        packets_data = packets_data_rest;
    }

    Ok((packets, rest))
}

fn parse_packets_n_items(value: &Bits) -> ParseResult<Vec<Packet>> {
    let (packets_count, mut rest) = value.split_at(11);
    let packets_count = packets_count.load_be::<usize>();

    let packets: Vec<_> = (0..packets_count)
        .map(|_| {
            let (packet, packets_data_rest) = parse_packet(rest)?;

            rest = packets_data_rest;

            Ok(packet)
        })
        .collect::<Result<_, ParsePacketError>>()?;

    Ok((packets, rest))
}

fn parse_literal(mut value: &Bits) -> ParseResult<u64> {
    let mut result: u64 = 0;

    loop {
        let (bit_flag, rest) = value.split_at(1);
        let (value_part, rest) = rest.split_at(4);

        result <<= 4;
        result |= value_part.load_be::<u64>();

        if bit_flag.all() {
            value = rest;
        } else {
            break Ok((result, rest));
        }
    }
}
