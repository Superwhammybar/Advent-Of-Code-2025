use std::{fs::read_to_string, path::Path};

pub fn get_day_data(path: &Path) -> Vec<String> {
    read_to_string(path)
        .unwrap()
        .split("\r\n")
        .map(|s| s.to_owned())
        .collect::<Vec<String>>()
}
