use aoc2022::read_string;
use std::collections::HashSet;
use std::char;

fn search(input: &String, length: usize) -> usize {
    let characters: Vec<char> = input.chars().collect();
    let count: usize = input.len();
    let mut i: usize = 0;
    while i < (count - length) {
        let subset: HashSet<char> = HashSet::from_iter(characters[i..i+length].to_vec());
        if subset.len() == length {
            return i + length;
        }
        i += 1;
    }

    0
}

fn part_1(input: &String) -> usize {
    search(input, 4)
}

fn part_2(input: &String) -> usize {
    search(input, 14)
}

fn main() {
    let input = read_string("inputs/day_06.txt");

    println!("Part 1 Results: {}", part_1(&input));
    println!("Part 2 Results: {}", part_2(&input));
}


#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_part_1() {
        let input = read_string("inputs/test/day_06.txt");

        assert_eq!(part_1(&input), 7);
    }

    #[test]
    fn test_part_2() {
        let input = read_string("inputs/test/day_06.txt");

        assert_eq!(part_2(&input), 19);
    }
}