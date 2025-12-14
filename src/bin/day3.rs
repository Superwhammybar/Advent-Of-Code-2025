use aoc_2025::data_parse::get_day_data;
use std::{
    ops::{Add, Mul},
    path::Path,
};

fn main() {
    let day_path = Path::new("./src/data/day3.txt");
    let day_data = get_day_data(day_path)
        .iter()
        .map(Bank::from)
        .collect::<Vec<Bank>>();

    let mut p1 = 0_u32;
    for mut bank in day_data {
        bank.process();
        let value = bank.largest_tens * 10 + bank.largest_digit;
        p1 += value.0 as u32;
    }
    println!("Part 1: {p1}");
}

#[derive(Clone, Copy, PartialEq, PartialOrd, Debug)]
struct Joltage(u8);
impl Mul<u8> for Joltage {
    type Output = Joltage;

    fn mul(self, rhs: u8) -> Self::Output {
        Joltage(self.0 * rhs)
    }
}
impl Add<Joltage> for Joltage {
    type Output = Joltage;

    fn add(self, rhs: Joltage) -> Self::Output {
        Joltage(self.0 + rhs.0)
    }
}

struct Bank {
    joltages: Vec<Joltage>,
    largest_tens: Joltage,
    largest_digit: Joltage,
}

impl From<&String> for Bank {
    fn from(value: &String) -> Self {
        let joltages = value
            .chars()
            .map(|v| Joltage(v.to_digit(10).unwrap() as u8));
        Bank {
            joltages: joltages.collect::<Vec<Joltage>>(),
            largest_tens: Joltage(0),
            largest_digit: Joltage(0),
        }
    }
}

impl Bank {
    fn process(&mut self) {
        for i in 0..self.joltages.len() - 1 {
            let base_joltage = self.joltages.get(i).unwrap();
            if base_joltage >= &self.largest_tens {
                for j in (i + 1)..self.joltages.len() {
                    let digit_joltage = self.joltages.get(j).unwrap();
                    if base_joltage == &self.largest_tens && digit_joltage > &self.largest_digit
                        || base_joltage > &self.largest_tens
                    {
                        self.largest_tens = *base_joltage;
                        self.largest_digit = *digit_joltage;
                    }
                }
            }
        }
    }
}
