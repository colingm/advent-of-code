use std::{fs, str};
use pom::char_class::{alpha, digit};
use pom::parser::{is_a, one_of, Parser};

pub fn read_string(filename: &str) -> String {
    fs::read_to_string(filename)
        .expect("Something went wrong reading the file")
}

pub fn integer<'a>() -> Parser<'a, u8, usize> {
    is_a(digit).repeat(1..)
        .collect()
        .convert(str::from_utf8)
        .convert(|s| s.parse::<usize>())
}

pub fn string<'a>() -> Parser<'a, u8, String> {
    (is_a(alpha) | one_of(b".-/")).repeat(1..)
        .collect()
        .convert(str::from_utf8)
        .map(|s| s.to_string())
}
