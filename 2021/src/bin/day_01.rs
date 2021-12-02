use aoc2021::get_input;
use aoc2021::parse_usize;

fn part_1(original: &[usize]) -> usize {
    let mut differences: usize = 0;
    let mut prev_value: usize = original[0];
    original.iter()
        .for_each(|value| {
            if value > &prev_value {
                differences += 1;
            }

            prev_value = *value;
        });

    differences
}

fn part_2(original: &[usize]) -> usize {
    let mut differences: usize = 0;
    let mut prev_value: usize = 0;
    original.iter()
        .enumerate()
        .for_each(|(i, value)| {
            if i < 3 {
                prev_value += value;
            } else {
                let current_value = prev_value - original[i - 3] + value;
                if current_value > prev_value {
                    differences += 1;
                }
                prev_value = current_value;
            }
        });

    differences
}

fn main() {
    let input = get_input("inputs/day_01.txt", &parse_usize);

    println!("Part 1 Results: {}", part_1(&input));
    println!("Part 2 Results: {}", part_2(&input));
}


#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_part_1() {
        let input = get_input("inputs/test/day_01.txt", &parse_usize);

        assert_eq!(part_1(&input), 7);
    }

    #[test]
    fn test_part_2() {
        let input = get_input("inputs/test/day_01.txt", &parse_usize);

        assert_eq!(part_2(&input), 5);
    }
}