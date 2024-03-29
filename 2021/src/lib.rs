use std::fs;

pub fn parse_usize(line: &str) -> usize {
    line.parse().unwrap()
}

pub fn parse_string(line: &str) -> String {
    line.to_string()
}

pub fn parse_u8_vec(line: &str) -> Vec<u8> {
    line.chars().map(|c| c.to_digit(10).unwrap() as u8).collect()
}

pub fn get_input<T>(filename: &str, map_fn: &dyn Fn(&str) -> T) -> Vec<T> {
    let contents = fs::read_to_string(filename)
        .expect("Something went wrong reading the file");

    contents
        .lines()
        .map(map_fn)
        .collect()
}
