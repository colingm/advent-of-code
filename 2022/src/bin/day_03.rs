use aoc2022::read_string;
use std::collections::HashSet;
use std::iter::FromIterator;
use pom::char_class::*;
use pom::parser::*;

fn parse(input: &String) -> Vec<Vec<u8>> {
    let item = is_a(alpha)
        .map(|i| if i > 96 { i - 96 } else { i - 38 });
    let rucksack = item.repeat(1..);
    let inventory = list(rucksack, sym(b'\n'));

    let parsed_inventory = inventory.parse(input.as_bytes())
        .unwrap();

    parsed_inventory
}

fn part_1(input: &String) -> usize {
    let inventory = parse(input);
    let priorities: Vec<usize> = inventory.iter()
        .map(|r| {
            let compartments: Vec<&[u8]> = r.chunks(r.len() / 2).collect();
            let compartment_sets: Vec<HashSet<&u8>> = compartments.iter()
                .map(|c| HashSet::from_iter(*c))
                .collect();
            let duplicate = compartment_sets[0].intersection(&compartment_sets[1]).last().unwrap();

            **duplicate as usize
        })
        .collect();

    priorities.iter()
        .sum()
}

fn part_2(input: &String) -> usize {
    let inventory = parse(input);
    let priorities: Vec<usize> = inventory.chunks(3)
        .map(|c| {
            let rucksacks: Vec<HashSet<&u8>> = c.iter()
                .map(|r| HashSet::from_iter(r))
                .collect();

            let id = rucksacks[0].intersection(&rucksacks[1])
                .find(|i| rucksacks[2].contains(**i))
                .unwrap();

            **id as usize
        })
        .collect();

    priorities.iter()
        .sum()
}

fn main() {
    let input = read_string("inputs/day_03.txt");

    println!("Part 1 Results: {}", part_1(&input));
    println!("Part 2 Results: {}", part_2(&input));
}


#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_part_1() {
        let input = read_string("inputs/test/day_03.txt");

        assert_eq!(part_1(&input), 157);
    }

    #[test]
    fn test_part_2() {
        let input = read_string("inputs/test/day_03.txt");

        assert_eq!(part_2(&input), 70);
    }
}