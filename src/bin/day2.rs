use aoc_2025::data_parse::get_day_data;
use std::path::Path;

fn main() {
    let day_path = Path::new("./src/data/day2.txt");
    let day_data = Ranges::from(get_day_data(day_path));

    let (part1, part2) = complete(day_data.clone());
    println!("Part 1: {part1}");
    println!("Part 2: {part2}");
}

fn complete(data: Ranges) -> (i64, i64) {
    let mut data = data;
    while !data.finished {
        data.step();
    }
    (data.p1_invalid_id_total, data.p2_invalid_id_total)
}

#[derive(Clone)]
struct Ranges {
    ranges: Vec<Range>,
    step_count: u32,
    finished: bool,
    p1_invalid_id_total: i64,
    p2_invalid_id_total: i64,
}

impl From<Vec<String>> for Ranges {
    fn from(value: Vec<String>) -> Self {
        let ranges = value[0].split(",");
        Ranges {
            ranges: ranges.map(Range::from).collect::<Vec<Range>>(),
            step_count: 0,
            finished: false,
            p1_invalid_id_total: 0,
            p2_invalid_id_total: 0,
        }
    }
}

impl Ranges {
    fn step(&mut self) {
        let current_range = self.ranges.get(self.step_count as usize);
        match current_range {
            Some(range) => {
                for i in range.start..=range.end {
                    let point = RangePoint(i);
                    if point.is_p1_invalid() {
                        self.p1_invalid_id_total += i;
                    }
                    if point.is_p2_invalid() {
                        self.p2_invalid_id_total += i;
                    }
                }
                self.step_count += 1;
            }
            None => {
                self.finished = true;
            }
        }
    }
}

struct RangePoint(i64);
impl RangePoint {
    fn is_odd_characters(&self) -> bool {
        self.0.to_string().len() % 2 != 0
    }

    fn is_p1_repeating_pattern(&self) -> bool {
        let mut str = self.0.to_string();
        let length = str.len();
        let half = str.split_off(length / 2);
        str == half
    }

    fn is_p1_invalid(&self) -> bool {
        if self.is_odd_characters() {
            return false;
        }
        if !self.is_p1_repeating_pattern() {
            return false;
        }
        true
    }

    fn is_p2_invalid(&self) -> bool {
        let str = self.0.to_string();
        let str_len = str.len();
        for i in 0..str_len / 2 {
            let slice = str.get(0..i + 1).unwrap();
            let full_slice = slice.repeat(str_len / (i + 1));
            if full_slice == str {
                return true;
            }
        }
        false
    }
}

#[derive(Clone)]
struct Range {
    start: i64,
    end: i64,
}

impl From<&str> for Range {
    fn from(value: &str) -> Self {
        let range_points = value.split("-").collect::<Vec<&str>>();
        Range {
            start: range_points[0].parse::<i64>().unwrap(),
            end: range_points[1].parse::<i64>().unwrap(),
        }
    }
}
