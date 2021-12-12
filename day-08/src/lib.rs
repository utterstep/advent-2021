use std::{error::Error, str::FromStr};

use advent_utils::{parse_raw_data, Part, Solver};

#[derive(Debug)]
pub struct Solution {
    signals: Vec<String>,
}

const OUTPUT_DIGITS_NUMBER: usize = 4;

impl FromStr for Solution {
    type Err = Box<dyn Error>;

    fn from_str(input_data: &str) -> Result<Self, Self::Err> {
        let signals = parse_raw_data(input_data)?;

        Ok(Self { signals })
    }
}

fn digit_to_segments_map(digit: &str) -> u8 {
    digit
        .bytes()
        .fold(0, |map, segment| map | (1 << (segment - b'a')))
}

fn deduce_signal_scheme(signal: &str) -> Option<u32> {
    let (digits, output) = signal.split_once('|').unwrap();
    let digits = digits
        .split_ascii_whitespace()
        .map(digit_to_segments_map)
        .collect::<Vec<_>>();

    let one = *digits.iter().find(|&digit| digit.count_ones() == 2)?;
    let four = *digits.iter().find(|&digit| digit.count_ones() == 4)?;
    let seven = *digits.iter().find(|&digit| digit.count_ones() == 3)?;
    let eight = (1u8 << 7) - 1;

    let a_segment = 1u8 << (seven ^ one).trailing_zeros();

    let nine_without_g = four | a_segment;
    let nine = *digits
        .iter()
        .find(|&&digit| digit > nine_without_g && (digit ^ nine_without_g).count_ones() == 1)?;
    let g_segment = nine ^ nine_without_g;
    let e_segment = eight ^ nine;

    let two_without_dc = a_segment | e_segment | g_segment;

    let two = *digits
        .iter()
        .find(|&&digit| digit > two_without_dc && (digit ^ two_without_dc).count_ones() == 2)?;

    let c_segment = two & one;
    let f_segment = one ^ c_segment;
    let d_segment = two ^ (two_without_dc | c_segment);

    let zero = eight ^ d_segment;

    let three = two ^ e_segment | f_segment;
    let five = nine ^ c_segment;
    let six = five | e_segment;

    let digits_array = [zero, one, two, three, four, five, six, seven, eight, nine];

    let mut result = 0;

    for (i, digit_map) in output
        .split_ascii_whitespace()
        .map(digit_to_segments_map)
        .enumerate()
    {
        let digit = digits_array.iter().position(|&digit| digit == digit_map)? as u32;

        result += digit * 10u32.pow((OUTPUT_DIGITS_NUMBER - i - 1) as u32);
    }

    Some(result)
}

impl Solver for Solution {
    fn solve(&self, part: Part) -> String {
        match part {
            Part::One => {
                let easy = self
                    .signals
                    .iter()
                    .map(|signal| {
                        let (_digits, output) = signal.split_once('|').unwrap();

                        output
                            .split_whitespace()
                            .filter(|&digit| matches!(digit.len(), 2 | 3 | 4 | 7))
                            .count()
                    })
                    .sum::<usize>();

                format!("there are {} easy digits", easy)
            }
            Part::Two => {
                let sum = self
                    .signals
                    .iter()
                    .map(|signal| deduce_signal_scheme(signal).expect("failed to deduce"))
                    .sum::<u32>();

                format!("sum of output signals is {}", sum)
            }
        }
    }

    fn day_number() -> u32 {
        8
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_first_signal() {
        assert_eq!(
            deduce_signal_scheme("acedgfb cdfbe gcdfa fbcad dab cefabd cdfgeb eafb cagedb ab | cdfeb fcadb cdfeb cdbaf"),
            Some(5353),
        )
    }

    #[test]
    fn test_first_failure() {
        assert_eq!(
            deduce_signal_scheme("aecbfdg fbg gf bafeg dbefa fcge gcbea fcaegb dgceab fcbdga | gecf egdcabf bgf bfgea"),
            Some(4873),
        )
    }

    #[test]
    fn test_short_example() {
        let input = include_str!("../short.txt");

        assert_eq!(
            input
                .split('\n')
                .map(|signal| deduce_signal_scheme(signal).expect("failed to deduce"))
                .sum::<u32>(),
            61229
        );
    }
}
