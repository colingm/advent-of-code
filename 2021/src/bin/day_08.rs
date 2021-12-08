use aoc2021::{get_input, parse_string};
use std::collections::{HashMap, HashSet};

// 0 - 6 segments
// 1 - 2 segments
// 2 - 5 segments
// 3 - 5 segments
// 4 - 4 segments
// 5 - 5 segments
// 6 - 6 segments
// 7 - 3 segments
// 8 - 7 segments
// 9 - 6 segments

// 2 segments = 1
// 3 segments = 7
// 4 segments = 4
// 5 segments = 2, 3, 5
// 6 segments = 0, 6, 9
// 7 segments = 8

fn part_1(lines: &Vec<String>) -> usize {
    let mut simple_digit_count: usize = 0;
    for line in lines.iter() {
        let output: &str = line.split('|').collect::<Vec<&str>>()[1];

        simple_digit_count += output.split_whitespace()
            .map(|digit| digit.trim())
            .filter(|digit| {
                let length = digit.len();

                length == 2 || length == 4 || length == 3 || length == 7
            })
            .count();
    }

    simple_digit_count
}

fn part_2(lines: &Vec<String>) -> usize {
    let mut output_sum: usize = 0;
    for line in lines.iter() {
        let split: Vec<&str> = line.split('|').collect::<Vec<&str>>();
        let input: &str = split[0];
        let output: &str = split[1];
        let mut unknown_digits: Vec<HashSet<char>> = Vec::new();
        let mut known_digits: HashMap<u8, HashSet<char>> = HashMap::new();

        for digit in input.split_whitespace() {
            let digit_chars: HashSet<char> = digit.chars().collect();
            match digit_chars.len() {
                2 => { known_digits.insert(1, digit_chars); },
                3 => { known_digits.insert(7, digit_chars); },
                4 => { known_digits.insert(4, digit_chars); },
                7 => { known_digits.insert(8, digit_chars); },
                _ => { unknown_digits.push(digit_chars); },
            };
        }

        for digit in unknown_digits.iter() {
            match digit.len() {
                5 => {
                    if digit.intersection(&known_digits[&4]).count() == 2 {
                        known_digits.insert(2, digit.clone());
                    } else if digit.intersection(&known_digits[&4]).count() == 3 && digit.intersection(&known_digits[&1]).count() == 2 {
                        known_digits.insert(3, digit.clone());
                    } else {
                        known_digits.insert(5, digit.clone());
                    }
                },
                6 => {
                    if digit.intersection(&known_digits[&1]).count() == 1 {
                        known_digits.insert(6, digit.clone());
                    } else if digit.intersection(&known_digits[&4]).count() == 3 {
                        known_digits.insert(0, digit.clone());
                    } else {
                        known_digits.insert(9, digit.clone());
                    }
                },
                _ => {}
            }
        }

        let mut number_string: String = String::new();
        for digit in output.split_whitespace() {
            let digit_chars: HashSet<char> = digit.chars().collect();
            let digit_length: usize = digit_chars.len();
            let (actual, _) = known_digits.iter()
                .find(|(_, known_chars)| known_chars.len() == digit_length && digit_chars.intersection(known_chars).count() == digit_length)
                .unwrap();
            number_string += actual.to_string().as_str();
        }

        let output_value: usize = number_string.parse().unwrap();
        output_sum += output_value;
    }

    output_sum
}

fn main() {
    let input = get_input("inputs/day_08.txt", &parse_string);

    println!("Part 1 Results: {}", part_1(&input));
    println!("Part 2 Results: {}", part_2(&input));
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_part_1() {
        let input = get_input("inputs/test/day_08.txt", &parse_string);

        assert_eq!(part_1(&input), 26);
    }

    #[test]
    fn test_part_1_answer() {
        let input = get_input("inputs/day_08.txt", &parse_string);

        assert_eq!(part_1(&input), 255);
    }

    #[test]
    fn test_part_2() {
        let input = get_input("inputs/test/day_08.txt", &parse_string);

        assert_eq!(part_2(&input), 61229);
    }

    #[test]
    fn test_part_2_answer() {
        let input = get_input("inputs/day_08.txt", &parse_string);

        assert_eq!(part_2(&input), 982158);
    }
}