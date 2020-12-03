use lazy_static::lazy_static;
use regex::Regex;
use std::fs;

lazy_static! {
    static ref POLICY_RE: Regex = Regex::new(r"(\d+)-(\d+) (\w): (\w*)").unwrap();
}

pub struct Policy {
    first_num: usize,
    second_num: usize,
    character: char,
    password: String,
}

fn parse_line(line: &str) -> Option<Policy> {
    let results = POLICY_RE.captures(line)?;

    Some(Policy {
        first_num: results.get(1)?.as_str().parse().ok()?,
        second_num: results.get(2)?.as_str().parse().ok()?,
        character: results.get(3)?.as_str().chars().next()?,
        password: results.get(4)?.as_str().to_string(),
    })
}

fn get_passwords(filename: &str) -> Vec<Policy> {
    let contents = fs::read_to_string(filename)
        .expect("Something went wrong reading the file");
    let passwords: Vec<Policy> = contents
        .lines()
        .map(|l| l.trim())
        .filter(|l| !l.is_empty())
        .filter_map(|l| parse_line(l))
        .collect();

    passwords
}

fn count_valid(policies: &Vec<Policy>) -> usize {
    policies.iter()
        .filter(|policy| {
            let count = &(policy.password).chars()
                .filter(|c| c == &policy.character)
                .count();

            &policy.first_num <= count && count <= &policy.second_num
        })
        .count()
}

fn count_real(policies: &Vec<Policy>) -> usize {
    policies.iter()
        .filter(|policy| {
            let first_val = policy.password.chars().nth(policy.first_num - 1).unwrap();
            let second_val = policy.password.chars().nth(policy.second_num - 1).unwrap();

            (first_val == policy.character) ^ (second_val == policy.character)
        })
        .count()
}

fn main() {
    let passwords = get_passwords("input.txt");

    println!("Part 1 Result: {}", count_valid(&passwords));
    println!("Part 2 Result: {}", count_real(&passwords));
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_first_policy() {
        let passwords = get_passwords("test.txt");

        assert_eq!(count_valid(&passwords), 2);
    }

    #[test]
    fn test_real_policy() {
        let passwords = get_passwords("test.txt");

        assert_eq!(count_real(&passwords), 1);
    }
}
