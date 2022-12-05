use aoc2022::{read_string, integer};
use itertools::Itertools;
use pom::parser::*;

fn parse(input: &String) -> Vec<usize> {
    let pack = list(integer(), sym(b'\n'));
    let elves = list(pack, sym(b'\n'));

    let inventory = elves.parse(input.as_bytes())
        .unwrap();

    inventory.iter()
        .map(|elf| elf.iter().sum::<usize>())
        .collect()
}

fn part_1(input: &String) -> usize {
    let elves = parse(input);
    *elves.iter()
        .max()
        .unwrap()
}

fn part_2(input: &String) -> usize {
    let elves = parse(input);
    elves.iter()
        .sorted()
        .rev()
        .take(3)
        .sum()
}

fn main() {
    let input = read_string("inputs/day_01.txt");

    println!("Part 1 Results: {}", part_1(&input));
    println!("Part 2 Results: {}", part_2(&input));
}


#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_part_1() {
        let input = read_string("inputs/test/day_01.txt");

        assert_eq!(part_1(&input), 24000);
    }

    #[test]
    fn test_part_2() {
        let input = read_string("inputs/test/day_01.txt");

        assert_eq!(part_2(&input), 45000);
    }
}