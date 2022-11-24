use std::{error::Error, str::FromStr};

use advent_utils::{parse_raw_data, Part, Solver};

#[derive(Debug)]
pub struct Solution {
    reports: Vec<String>,
    width: usize,
}

impl FromStr for Solution {
    type Err = Box<dyn Error>;

    fn from_str(input_data: &str) -> Result<Self, Self::Err> {
        let reports: Vec<String> = parse_raw_data(input_data)?;

        Ok(Self {
            width: reports[0].len(),
            reports,
        })
    }
}

impl Solver for Solution {
    fn solve(&self, part: Part) -> String {
        match part {
            Part::One => {
                let (zeroes, ones) = {
                    let mut zeroes = vec![0; self.width];
                    let mut ones = vec![0; self.width];

                    for report in &self.reports {
                        for (i, b) in report.bytes().enumerate() {
                            match b {
                                b'0' => zeroes[i] += 1,
                                b'1' => ones[i] += 1,
                                _ => unreachable!(),
                            }
                        }
                    }

                    (zeroes, ones)
                };

                let iter = zeroes
                    .into_iter()
                    .rev()
                    .zip(ones.into_iter().rev())
                    .enumerate();
                let mut gamma = 0;
                let mut epsilon_rate = 0;

                for (i, (n_zeroes, n_ones)) in iter {
                    if n_zeroes > n_ones {
                        epsilon_rate |= 1 << i;
                    } else {
                        gamma |= 1 << i;
                    }
                }

                format!("power consumption is {}", gamma * epsilon_rate)
            }
            Part::Two => {
                let mut o2 = Vec::with_capacity(self.width);
                let mut co2 = Vec::with_capacity(self.width);

                let mut o2_reports = self
                    .reports
                    .iter()
                    .map(|s| s.as_bytes())
                    .collect::<Vec<_>>();
                let mut co2_reports = self
                    .reports
                    .iter()
                    .map(|s| s.as_bytes())
                    .collect::<Vec<_>>();

                for i in 0..self.width {
                    if o2_reports.len() > 1 {
                        let (n_zeroes, n_ones) = o2_reports.iter().fold(
                            (0, 0),
                            |(zeroes, ones), &report| match report[i] {
                                b'0' => (zeroes + 1, ones),
                                b'1' => (zeroes, ones + 1),
                                _ => unreachable!(),
                            },
                        );

                        if n_zeroes > n_ones {
                            o2.push(b'0');
                        } else {
                            o2.push(b'1');
                        }

                        o2_reports.retain(|&s| s.starts_with(&o2));
                    }

                    if co2_reports.len() > 1 {
                        let (n_zeroes, n_ones) =
                            co2_reports
                                .iter()
                                .fold((0, 0), |(zeroes, ones), &report| match report[i] {
                                    b'0' => (zeroes + 1, ones),
                                    b'1' => (zeroes, ones + 1),
                                    _ => unreachable!(),
                                });

                        if n_zeroes > n_ones {
                            co2.push(b'1');
                        } else {
                            co2.push(b'0');
                        }

                        co2_reports.retain(|&s| s.starts_with(&co2));
                    }
                }

                let o2 = match o2_reports[..] {
                    [report] => usize::from_str_radix(std::str::from_utf8(report).unwrap(), 2),
                    _ => return "not found suitable O2 report".to_owned(),
                };

                let co2 = match co2_reports[..] {
                    [report] => usize::from_str_radix(std::str::from_utf8(report).unwrap(), 2),
                    _ => return "not found suitable O2 report".to_owned(),
                };

                match (o2, co2) {
                    (Ok(o2), Ok(co2)) => format!("life support rating is {}", o2 * co2),
                    _ => "failed to parse ratings".to_owned(),
                }
            }
        }
    }

    fn day_number() -> u32 {
        3
    }
}
