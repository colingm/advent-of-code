use std::fs;

pub fn parse_usize(line: &str) -> usize {
    line.parse().unwrap()
}

pub fn get_input<T>(filename: &str, map_fn: &dyn Fn(&str) -> T) -> Vec<T> {
    let contents = fs::read_to_string(filename)
        .expect("Something went wrong reading the file");

    contents
        .lines()
        .map(map_fn)
        .collect()
}