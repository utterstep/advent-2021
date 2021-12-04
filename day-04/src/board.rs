use std::{num::ParseIntError, str::FromStr};

const SIDE: usize = 5;

#[derive(Debug, Clone)]
pub struct Board {
    numbers: [u32; SIDE * SIDE],
    marked: [bool; SIDE * SIDE],
    row_marks: [usize; SIDE],
    col_marks: [usize; SIDE],
    won: bool,
}

impl FromStr for Board {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let numbers: Vec<u32> = s
            .split_ascii_whitespace()
            .map(str::parse)
            .collect::<Result<_, _>>()?;

        Ok(Self {
            won: false,
            numbers: numbers.try_into().expect("bingo board of invalid size"),
            marked: [false; SIDE * SIDE],
            row_marks: [0; SIDE],
            col_marks: [0; SIDE],
        })
    }
}

impl Board {
    /// process called number, returning true, if this was a winning move for this board
    pub fn process_call(&mut self, called_number: u32) -> bool {
        if self.won {
            return false;
        }

        for (i, n) in self.numbers.iter().enumerate() {
            if *n == called_number {
                let row = i / SIDE;
                let col = i % SIDE;

                self.marked[i] = true;
                self.col_marks[col] += 1;
                self.row_marks[row] += 1;

                if self.col_marks[col] == SIDE || self.row_marks[row] == SIDE {
                    self.won = true;
                    return true;
                }

                break;
            }
        }

        false
    }

    pub fn unmarked_sum(&self) -> u32 {
        self.numbers
            .iter()
            .enumerate()
            .filter_map(|(i, n)| (!self.marked[i]).then(|| *n))
            .sum()
    }
}
