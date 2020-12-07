use lazy_static::lazy_static;
use regex::Regex;
use std::fs;
use std::collections::HashMap;

lazy_static! {
    static ref RULE_RE: Regex = Regex::new(r"^([\w ]+) bags contain (.*\.)$").unwrap();
    static ref CONTAINED_RE: Regex = Regex::new(r"(\d) ([\w ]+) bags?").unwrap();
}

fn contains_gold(color: &String, bags: &HashMap<String, HashMap<String, usize>>) -> bool {
    if bags[color].contains_key(&"shiny gold".to_string()) {
        return true;
    }

    bags[color].iter()
        .any(|(col, _)| contains_gold(col, bags))
}

fn part_1(input: &HashMap<String, HashMap<String, usize>>) -> usize {
    input.iter()
        .filter(|(color, _)| {
            let contains = contains_gold(color, input);
            contains
        })
        .count()
}

fn total_bags(color: &String, input: &HashMap<String, HashMap<String, usize>>) -> usize {
    1 + input[color].iter()
            .map(|(b, count)| count * total_bags(b, input))
            .sum::<usize>()
}

fn part_2(input: &HashMap<String, HashMap<String, usize>>) -> usize {
    total_bags(&"shiny gold".to_string(), input) - 1
}

fn get_input(filename: &str) -> HashMap<String, HashMap<String, usize>> {
    let contents = fs::read_to_string(filename)
        .expect("Something went wrong reading the file");

    contents
        .lines()
        .map(|line| {
            let results = RULE_RE.captures(line).unwrap();
            let bag: String = results.get(1).unwrap().as_str().to_string();
            let contained: &str = results.get(2).unwrap().as_str();
            let mut inside: HashMap<String, usize> = HashMap::new();

            if !contained.contains("no") {
                inside = contained.split(',')
                    .map(|b| {
                        let matched = CONTAINED_RE.captures(b).unwrap();
                        let count = matched.get(1).unwrap().as_str().parse().unwrap();
                        let color = matched.get(2).unwrap().as_str().to_string();
                        (color, count)
                    })
                    .collect();
            }

            (bag, inside)
        })
        .collect()
}

fn main() {
    let input = get_input("input.txt");

    println!("Part 1 Results: {}", part_1(&input));
    println!("Part 2 Results: {}", part_2(&input));
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_part_1() {
        let test_input = get_input("test.txt");

        assert_eq!(part_1(&test_input), 4);
    }

    #[test]
    fn test_part_2() {
        let test_input = get_input("test.txt");

        assert_eq!(part_2(&test_input), 32);
    }
}
