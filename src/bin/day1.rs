use aoc_2025::data_parse::get_day_data;
use std::path::Path;

fn main() {
    let day_path = Path::new("./src/data/day1.txt");
    let day_data = Instructions::from(get_day_data(day_path));

    let (part1, part2) = complete(day_data.clone());
    println!("Part 1: {part1}");
    println!("Part 2: {part2}");
}

fn complete(data: Instructions) -> (u32, u32) {
    let mut instructions = data;
    while !instructions.finished {
        instructions.step();
    }
    (instructions.is_zero, instructions.pass_through_zero)
}

#[derive(Clone)]
enum Direction {
    L,
    R,
}

#[derive(Clone)]
struct Instructions {
    steps: Vec<Instruction>,
    step_count: u32,
    pass_through_zero: u32,
    is_zero: u32,
    current_number: i64,
    finished: bool,
}

impl Instructions {
    fn step(&mut self) {
        let maybe_instruction = self.steps.get(self.step_count as usize);
        match maybe_instruction {
            Some(instruction) => {
                let change = instruction.change();
                let mut changing_number = self.current_number;
                for _ in 0..change.abs() {
                    changing_number += if change < 0 { -1 } else { 1 };

                    if changing_number < 0 {
                        changing_number = 99;
                    } else if changing_number > 99 {
                        changing_number = 0;
                    }
                    if changing_number == 0 {
                        self.pass_through_zero += 1;
                    }
                }
                if changing_number == 0 {
                    self.is_zero += 1;
                }
                self.current_number = changing_number;
            }
            None => self.finished = true,
        }
        self.step_count += 1;
    }
}

#[derive(Clone)]
struct Instruction {
    direction: Direction,
    value: i64,
}

impl Instruction {
    fn change(&self) -> i64 {
        match self.direction {
            Direction::L => -(self.value),
            Direction::R => self.value,
        }
    }
}

impl From<char> for Direction {
    fn from(value: char) -> Self {
        match value {
            'L' => Direction::L,
            'R' => Direction::R,
            _ => unreachable!(),
        }
    }
}

impl From<&String> for Instruction {
    fn from(value: &String) -> Self {
        let mut chars = value.chars();
        let dir = chars.next().unwrap();
        let value = chars.collect::<String>().parse::<u32>().unwrap();
        Instruction {
            direction: Direction::from(dir),
            value: value.into(),
        }
    }
}

impl From<Vec<String>> for Instructions {
    fn from(value: Vec<String>) -> Self {
        Instructions {
            steps: value
                .iter()
                .map(Instruction::from)
                .collect::<Vec<Instruction>>(),
            step_count: 0,
            current_number: 50,
            finished: false,
            is_zero: 0,
            pass_through_zero: 0,
        }
    }
}
