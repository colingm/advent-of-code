use aoc2021::{get_input, parse_string};

const PAIRS: [(char, char, usize); 4] = [('(', ')', 3), ('[', ']', 57), ('{', '}', 1197), ('<', '>', 25137)];

fn part_1(rows: &Vec<String>) -> usize {
    let mut sum = 0;

    for row in rows {
        let mut symbols: Vec<char> = Vec::new();

        for symbol in row.chars() {
            let closed = PAIRS.iter().find(|&c| c.1 == symbol);
            match closed {
                Some(c) => {
                    let last = symbols.pop();
                    match last {
                        Some(l) => {
                            if l != c.0 {
                                sum += c.2;
                                break;
                            }
                        },
                        None => {}
                    }
                },
                None => {
                    symbols.push(symbol);
                }
            }
        }
    }

    sum
}

fn part_2(rows: &Vec<String>) -> usize {
    let mut row_values: Vec<usize> = Vec::new();

    for row in rows {
        let mut symbols: Vec<char> = Vec::new();
        let mut corrupt = false;
        let mut row_result = 0;
        for symbol in row.chars() {
            let closed = PAIRS.iter().find(|&c| c.1 == symbol);
            match closed {
                Some(close) => {
                    let last = symbols.pop();
                    match last {
                        Some(l) => {
                            if l != close.0 {
                                corrupt = true;
                                break;
                            }
                        },
                        None => {}
                    }
                },
                None => {
                    symbols.push(symbol);
                }
            }
        }

        if !corrupt {
            symbols.reverse();
            for &symbol in symbols.iter() {
                let opener = PAIRS.iter().position(|&c| c.0 == symbol);
                match opener {
                    Some(open_index) => {
                        row_result = row_result * 5 + open_index + 1;
                    },
                    None => {}
                }
            }
            row_values.push(row_result);
        }
    }

    row_values.sort();

    row_values[row_values.len() / 2]
}

fn main() {
    let input: Vec<String> = get_input("inputs/day_10.txt", &parse_string);

    println!("Part 1 Results: {}", part_1(&input));
    println!("Part 2 Results: {}", part_2(&input));
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_part_1() {
        let input = get_input("inputs/test/day_10.txt", &parse_string);

        assert_eq!(part_1(&input), 26397);
    }

    #[test]
    fn test_part_1_answer() {
        let input = get_input("inputs/day_10.txt", &parse_string);

        assert_eq!(part_1(&input), 343863);
    }

    #[test]
    fn test_part_2() {
        let input = get_input("inputs/test/day_10.txt", &parse_string);

        assert_eq!(part_2(&input), 288957);
    }

    #[test]
    fn test_part_2_answer() {
        let input = get_input("inputs/day_10.txt", &parse_string);

        assert_eq!(part_2(&input), 2924734236);
    }
}