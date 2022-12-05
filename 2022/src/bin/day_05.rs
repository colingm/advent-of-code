use aoc2022::{read_string, integer};
use std::collections::VecDeque;
use std::char;
use pom::char_class::*;
use pom::parser::*;

#[derive(Debug, Clone)]
struct Instruction {
    source: usize,
    destination: usize,
    count: usize
}

type Crates = Vec<VecDeque<char>>;

struct Manual {
    crates: Crates,
    instructions: Vec<Instruction>
}

fn parse(input: &String) -> Manual {
    let fruit_crate = sym(b'[') * is_a(alpha).map(|c| c as char) - sym(b']');
    let empty_spot = sym(b' ').map(|c| c as char) - sym(b' ') - sym(b' ');
    let crate_row = list(fruit_crate | empty_spot, sym(b' '));
    let all_crates = list(crate_row, sym(b'\n'));

    let number_row = (sym(b' ').repeat(1..) * integer() - sym(b'\n').opt()).repeat(1..);

    let count = seq(b"move ") * integer();
    let source = seq(b" from ") * integer();
    let destination = seq(b" to ") * integer();
    let instruction_row = (count + source + destination)
        .map(|r| Instruction { count: r.0.0, source: r.0.1, destination: r.1});
    let instructions = list(instruction_row, sym(b'\n'));

    let full_sheet = all_crates - number_row - sym(b'\n') + instructions;

    let result = full_sheet.parse(input.as_bytes())
        .unwrap();

    let mut crates: Crates = vec![];
    result.0.iter()
        .for_each(|r| {
            r.iter().enumerate()
                .for_each(|(i, c)| {
                    if *c != ' ' {
                        while i + 1 > crates.len() { crates.push(VecDeque::new()); }
                        crates[i].push_front(*c);
                    }
                });
        });

    Manual { crates: crates, instructions: result.1.to_vec() }
}

fn part_1(input: &String) -> String {
    let mut manual = parse(input);
    manual.instructions.iter()
        .for_each(|i| {
            (1..(i.count + 1))
                .for_each(|_| {
                    let letter = manual.crates[i.source - 1].pop_back().unwrap();
                    manual.crates[i.destination - 1].push_back(letter);
                })
        });

    let result: Vec<String> = manual.crates.iter_mut()
        .map(|c| {
            match c.pop_back() {
                Some(x) => x.to_string(),
                None => "".to_string()
            }
        })
        .collect();

    result.join("")
}

fn part_2(input: &String) -> String {
    let mut manual = parse(input);
    manual.instructions.iter()
        .for_each(|i| {
            let at = manual.crates[i.source - 1].len() - i.count;
            let mut temp = manual.crates[i.source - 1].split_off(at);
            manual.crates[i.destination - 1].append(&mut temp);
        });

    let result: Vec<String> = manual.crates.iter_mut()
        .map(|c| {
            match c.pop_back() {
                Some(x) => x.to_string(),
                None => "".to_string()
            }
        })
        .collect();

    result.join("")
}

fn main() {
    let input = read_string("inputs/day_05.txt");

    println!("Part 1 Results: {}", part_1(&input));
    println!("Part 2 Results: {}", part_2(&input));
}


#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_part_1() {
        let input = read_string("inputs/test/day_05.txt");

        assert_eq!(part_1(&input), "CMZ");
    }

    #[test]
    fn test_part_2() {
        let input = read_string("inputs/test/day_05.txt");

        assert_eq!(part_2(&input), "MCD");
    }
}