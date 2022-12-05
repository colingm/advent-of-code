use aoc2022::read_string;
use pom::char_class::*;
use pom::parser::*;

fn parse(input: &String) -> Vec<(u8, u8)> {
    let round = (is_a(alpha) - sym(b' ') + is_a(alpha) - sym(b'\n'))
        .map(|p| (p.0 - 64, p.1 - 23 - 64));
    let rounds = round.repeat(1..);

    let parsed_rounds = rounds.parse(input.as_bytes())
        .unwrap();

    parsed_rounds
}

fn part_1(input: &String) -> usize {
    let rounds = parse(input);
    let scores: Vec<usize> = rounds.iter()
        .map(|r| {
            let score: u8 = match r.1 as i8 - r.0 as i8 {
                1 | -2 => 6,
                0 => 3,
                -1 | 2 => 0,
                _ => 0
            };

            (score + r.1) as usize
        })
        .collect();

    scores.iter()
        .sum()
}

fn part_2(input: &String) -> usize {
    let rounds = parse(input);
    let scores: Vec<usize> = rounds.iter()
        .map(|r| {
            let outcome: (u8, u8) = match r.1 {
                1 => (0, if r.0 == 1 { 3 } else { r.0 - 1 }),
                2 => (3, r.0),
                3 => (6, if r.0 == 3 { 1 } else { r.0 + 1 }),
                _ => (0, 0)
            };

            (outcome.0 + outcome.1) as usize
        })
        .collect();

    scores.iter()
        .sum()
}

fn main() {
    let input = read_string("inputs/day_02.txt");

    println!("Part 1 Results: {}", part_1(&input));
    println!("Part 2 Results: {}", part_2(&input));
}


#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_part_1() {
        let input = read_string("inputs/test/day_02.txt");

        assert_eq!(part_1(&input), 15);
    }

    #[test]
    fn test_part_2() {
        let input = read_string("inputs/test/day_02.txt");

        assert_eq!(part_2(&input), 12);
    }
}