use aoc2022::read_string;
use std::collections::HashSet;
use std::iter::FromIterator;
use std::str;
use pom::char_class::*;
use pom::parser::*;

#[derive(Debug)]
struct Assignment(usize, usize);

#[derive(Debug)]
struct Pair(Assignment, Assignment);

fn parse(input: &String) -> Vec<Pair> {
    let integer = || (is_a(digit).repeat(1..).collect().convert(str::from_utf8).convert(|s| s.parse::<usize>()));
    let assignment = || (integer() - sym(b'-') + integer())
        .map(|a| Assignment(a.0, a.1));
    let pair = (assignment() - sym(b',') + assignment()).map(|p| Pair(p.0, p.1));
    let all_pairs = list(pair, sym(b'\n'));

    let jobs = all_pairs.parse(input.as_bytes()).unwrap();

    jobs
}

fn part_1(input: &String) -> usize {
    let pairs = parse(input);
    pairs.iter()
        .filter(|p| {
            let first: HashSet<usize> = HashSet::from_iter(p.0.0..(p.0.1 + 1));
            let second: HashSet<usize> = HashSet::from_iter(p.1.0..(p.1.1 + 1));
            let intersection = first.intersection(&second).count();

            intersection == first.len() || intersection == second.len()
        })
        .count()
}

fn part_2(input: &String) -> usize {
    let pairs = parse(input);
    pairs.iter()
        .filter(|p| {
            let first: HashSet<usize> = HashSet::from_iter(p.0.0..(p.0.1 + 1));
            let second: HashSet<usize> = HashSet::from_iter(p.1.0..(p.1.1 + 1));
            let intersection = first.intersection(&second).count();

            intersection > 0
        })
        .count()
}

fn main() {
    let input = read_string("inputs/day_04.txt");

    println!("Part 1 Results: {}", part_1(&input));
    println!("Part 2 Results: {}", part_2(&input));
}


#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_part_1() {
        let input = read_string("inputs/test/day_04.txt");

        assert_eq!(part_1(&input), 2);
    }

    #[test]
    fn test_part_2() {
        let input = read_string("inputs/test/day_04.txt");

        assert_eq!(part_2(&input), 4);
    }
}