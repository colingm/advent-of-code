use aoc2021::{get_input, parse_string};

fn part_1(lines: &Vec<String>) -> usize {
    let mut positions: Vec<usize> = lines.iter()
        .map(|line| line.split(",").map(|f| f.parse().unwrap()).collect::<Vec<usize>>())
        .flatten()
        .collect();

    positions.sort();
    let middle: usize = positions.len() / 2;
    let median: usize = positions[middle];

    positions.iter()
        .map(|pos| ((*pos as isize) - (median as isize)).abs() as usize)
        .sum()
}

fn part_2(lines: &Vec<String>) -> usize {
    let positions: Vec<usize> = lines.iter()
        .map(|line| line.split(",").map(|f| f.parse().unwrap()).collect::<Vec<usize>>())
        .flatten()
        .collect();

    let sum: usize = positions.iter().sum();
    let average: usize = (sum as f64 / positions.len() as f64).round() as usize;

    let plus1: usize = positions.iter()
        .map(|pos| ((*pos as isize) - ((average + 1) as isize)).abs() as usize)
        .map(|dist| dist * (dist + 1) / 2)
        .sum();

    let minus1: usize = positions.iter()
        .map(|pos| ((*pos as isize) - ((average - 1) as isize)).abs() as usize)
        .map(|dist| dist * (dist + 1) / 2)
        .sum();

    let middle: usize = positions.iter()
        .map(|pos| ((*pos as isize) - (average as isize)).abs() as usize)
        .map(|dist| dist * (dist + 1) / 2)
        .sum();

    *vec![minus1, middle, plus1].iter().min().unwrap()
}

fn main() {
    let input = get_input("inputs/day_07.txt", &parse_string);

    println!("Part 1 Results: {}", part_1(&input));
    println!("Part 2 Results: {}", part_2(&input));
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_part_1() {
        let input = get_input("inputs/test/day_07.txt", &parse_string);

        assert_eq!(part_1(&input), 37);
    }

    #[test]
    fn test_part_1_answer() {
        let input = get_input("inputs/day_07.txt", &parse_string);

        assert_eq!(part_1(&input), 347509);
    }

    #[test]
    fn test_part_2() {
        let input = get_input("inputs/test/day_07.txt", &parse_string);

        assert_eq!(part_2(&input), 168);
    }

    #[test]
    fn test_part_2_answer() {
        let input = get_input("inputs/day_07.txt", &parse_string);

        assert_eq!(part_2(&input), 98257206);
    }
}