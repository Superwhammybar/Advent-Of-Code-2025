use aoc_2025::data_parse::get_day_data;
use std::{
    ops::{Add, Mul},
    path::Path,
};

fn main() {
    let day_path = Path::new("./src/data/day3.txt");
    let p1_data = get_day_data(day_path)
        .iter()
        .map(|v| Bank::from(v, 2))
        .collect::<Vec<Bank>>();

    let mut p1 = 0_u32;
    for mut bank in p1_data {
        bank.process();
        let value = bank
            .current_digits
            .iter()
            .map(|v| v.0.to_string())
            .collect::<Vec<String>>()
            .join("")
            .parse::<u32>()
            .unwrap();
        p1 += value;
    }
    println!("Part 1: {p1}");

    let p2_data = get_day_data(day_path)
        .iter()
        .map(|v| Bank::from(v, 12))
        .collect::<Vec<Bank>>();

    let mut p2 = 0_u64;
    for mut bank in p2_data {
        bank.process();
        let value = bank
            .current_digits
            .iter()
            .map(|v| v.0.to_string())
            .collect::<Vec<String>>()
            .join("")
            .parse::<u64>()
            .unwrap();
        p2 += value;
    }
    println!("Part 2: {p2}");
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
    target_length: usize,
    digits_to_consider: usize,
    current_digits: Vec<Joltage>,
    latest_digit_index: usize,
}

impl Bank {
    fn from(value: &str, target_length: usize) -> Self {
        let joltages = value
            .chars()
            .map(|v| Joltage(v.to_digit(10).unwrap() as u8))
            .collect::<Vec<Joltage>>();
        Bank {
            joltages: joltages.clone(),
            target_length,
            digits_to_consider: joltages.len(),
            current_digits: vec![],
            latest_digit_index: 0,
        }
    }
}

impl Bank {
    fn process(&mut self) {
        while self.digits_to_consider != self.target_length && self.target_length > 0 {
            let search_size = self.digits_to_consider - self.target_length;
            let to_search = self
                .joltages
                .get(self.latest_digit_index..self.latest_digit_index + search_size + 1)
                .unwrap();
            // println!(
            //     "Current Digits: {:?}, Target Length: {}, Latest Digit Index: {}, Digits To Consider: {}, Checking Digits: {:?}, Searching: {:?}",
            //     self.current_digits,
            //     self.target_length,
            //     self.latest_digit_index,
            //     self.digits_to_consider,
            //     self.latest_digit_index..self.latest_digit_index + search_size,
            //     to_search
            // );
            let (mut largest_index, mut largest_value) = (0, 0);
            for s in 0..to_search.len() {
                let compare_value = to_search.get(s).unwrap();
                if compare_value > &Joltage(largest_value) {
                    largest_value = compare_value.0;
                    largest_index = s;
                }
            }
            self.current_digits.push(Joltage(largest_value));
            self.target_length -= 1;
            self.latest_digit_index = self.latest_digit_index + 1 + largest_index;
            self.digits_to_consider = self.joltages.len() - self.latest_digit_index;
        }
        if self.digits_to_consider == self.target_length {
            self.current_digits = [
                self.current_digits.clone(),
                self.joltages
                    .get(self.latest_digit_index..self.joltages.len())
                    .unwrap()
                    .to_vec(),
            ]
            .concat();
        }
        println!("{:?}", self.current_digits);
    }
}
