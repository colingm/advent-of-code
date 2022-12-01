use std::fs;

pub fn read_string(filename: &str) -> String {
    fs::read_to_string(filename)
        .expect("Something went wrong reading the file")
}
