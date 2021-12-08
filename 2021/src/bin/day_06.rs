use aoc2021::{get_input, parse_string};
use std::collections::HashMap;

const LANTERN_FISH_CYCLE: usize = 6;

#[derive(Debug)]
struct FishSchool {
    cycle_length: usize,
    fish_counts: HashMap<usize, usize>
}

impl FishSchool {
    pub fn new(cycle_length:usize, fish: &Vec<usize>) -> Self {
        let mut fish_counts: HashMap<usize, usize> = HashMap::new();

        for f in fish.iter() {
            *fish_counts.entry(*f).or_insert(0) += 1;
        }

        Self {
            cycle_length,
            fish_counts
        }
    }

    pub fn advance_time(&mut self) {
        let mut new_fish_counts: HashMap<usize, usize> = HashMap::new();
        for (days, fish) in self.fish_counts.iter() {
            if *days == 0 {
                *new_fish_counts.entry(self.cycle_length).or_insert(0) += fish;
                *new_fish_counts.entry(self.cycle_length + 2).or_insert(0) += fish;
            } else {
                *new_fish_counts.entry(days - 1).or_insert(0) += fish;
            }
        }

        self.fish_counts = new_fish_counts;
    }

    pub fn count_fish(&self) -> usize {
        self.fish_counts.iter()
            .map(|(_, fish)| fish)
            .sum()
    }
}

fn get_fish(lines: &Vec<String>) -> Vec<usize> {
    lines.iter()
        .map(|line| line.split(",").map(|f| f.parse().unwrap()).collect::<Vec<usize>>())
        .flatten()
        .collect()
}

fn part_1(lines: &Vec<String>) -> usize {
    let fish = get_fish(lines);
    let mut fish_school = FishSchool::new(LANTERN_FISH_CYCLE, &fish);
    for _ in 0..80 {
        fish_school.advance_time();
    }

    fish_school.count_fish()
}

fn part_2(lines: &Vec<String>) -> usize {
    let fish = get_fish(lines);
    let mut fish_school = FishSchool::new(LANTERN_FISH_CYCLE, &fish);
    for _ in 0..256 {
        fish_school.advance_time();
    }

    fish_school.count_fish()
}

fn main() {
    let input = get_input("inputs/day_06.txt", &parse_string);

    println!("Part 1 Results: {}", part_1(&input));
    println!("Part 2 Results: {}", part_2(&input));
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_part_1() {
        let input = get_input("inputs/test/day_06.txt", &parse_string);

        assert_eq!(part_1(&input), 5934);
    }

    #[test]
    fn test_part_2() {
        let input = get_input("inputs/test/day_06.txt", &parse_string);

        assert_eq!(part_2(&input), 26984457539);
    }

    #[test]
    fn test_part_1_answer() {
        let input = get_input("inputs/day_06.txt", &parse_string);

        assert_eq!(part_1(&input), 390011);
    }

    #[test]
    fn test_part_2_answer() {
        let input = get_input("inputs/day_06.txt", &parse_string);

        assert_eq!(part_2(&input), 1746710169834);
    }
}