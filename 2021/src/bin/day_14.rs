use aoc2021::{get_input, parse_string};
use std::collections::HashMap;
use regex::Regex;
use lazy_static::lazy_static;

lazy_static! {
    pub static ref PAIR_RE: Regex = Regex::new(r"(?P<in1>\w)(?P<in2>\w) -> (?P<out>\w+)").unwrap();
}

struct Polymer {
    elements: HashMap<String, usize>,
    pairs: HashMap<String, usize>,
    rules: HashMap<String, Vec<String>>
}

impl Polymer {
    fn new(rows: &Vec<String>) -> Self {
        let mut old_rows = rows.clone();
        let rule_rows: Vec<String> = old_rows.split_off(2);
        let value = old_rows[0].to_string();
        let mut elements: HashMap<String, usize> = HashMap::new();
        let mut pairs: HashMap<String, usize> = HashMap::new();
        let mut iter = value.chars();
        let mut prev: String = String::new();
        while let Some(next) = iter.next() {
            let next_char: String = next.to_string();
            *elements.entry(next_char.to_string()).or_insert(0) += 1;
            if !prev.is_empty() {
                let pair: String = prev + &next_char;
                *pairs.entry(pair).or_insert(0) += 1;
            }

            prev = next_char;
        }

        let mut rules: HashMap<String, Vec<String>> = HashMap::new();
        for rule in rule_rows {
            let cap = PAIR_RE.captures(rule.as_str()).unwrap();
            let in1 = cap["in1"].to_string();
            let in2 = cap["in2"].to_string();
            let out = cap["out"].to_string();
            let rule_out: Vec<String> = vec![in1.to_string() + &out, out + &in2];
            rules.insert(in1.to_string() + &in2, rule_out);
        }

        Self {
            elements,
            pairs,
            rules
        }
    }

    fn build(&mut self) {
        let mut new_pairs: HashMap<String, usize> = HashMap::new();
        for (pair, count) in &self.pairs {
            let first_pair = &self.rules[pair][0];
            *self.elements.entry(first_pair.chars().nth(1).unwrap().to_string()).or_insert(0) += count;
            for out in &self.rules[pair] {
                *new_pairs.entry(out.to_string()).or_insert(0) += count;
            }
        }

        self.pairs = new_pairs;
    }

    fn get_min_and_max(&self) -> (usize, usize) {

        let mut min: usize = 0;
        let mut max: usize = 0;
        for (i, &value) in self.elements.values().enumerate() {
            if i == 0 {
                min = value;
            }
            if value < min {
                min = value;
            }
            if value > max {
                max = value;
            }
        }

        (min, max)
    }
}

fn part_1(rows: &Vec<String>) -> usize {
    let mut polymer = Polymer::new(rows);

    for _ in 0..10 {
        polymer.build();
    }

    let (min, max): (usize, usize) = polymer.get_min_and_max();

    max - min
}

fn part_2(rows: &Vec<String>) -> usize {
    let mut polymer = Polymer::new(rows);

    for _ in 0..40 {
        polymer.build();
    }

    let (min, max): (usize, usize) = polymer.get_min_and_max();

    max - min
}

fn main() {
    let input: Vec<String> = get_input("inputs/day_14.txt", &parse_string);

    println!("Part 1 Results: {}", part_1(&input));
    println!("Part 2 Results: {}", part_2(&input));
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_part_1() {
        let input = get_input("inputs/test/day_14.txt", &parse_string);

        assert_eq!(part_1(&input), 1588);
    }

    #[test]
    fn test_part_1_answer() {
        let input = get_input("inputs/day_14.txt", &parse_string);

        assert_eq!(part_1(&input), 2549);
    }

    #[test]
    fn test_part_2() {
        let input = get_input("inputs/test/day_14.txt", &parse_string);

        assert_eq!(part_2(&input), 2188189693529);
    }

    #[test]
    fn test_part_2_answer() {
        let input = get_input("inputs/day_14.txt", &parse_string);

        assert_eq!(part_2(&input), 2516901104210);
    }
}