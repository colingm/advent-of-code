use aoc2022::read_string;
use pom::char_class::*;
use pom::parser::*;

const DIR: &'static [(isize, isize)] = &[(0, 1), (1, 0), (0, -1), (-1, 0)];

fn parse(input: &String) -> Vec<Vec<usize>> {
    let tree: Parser<u8, usize> = is_a(digit).map(|d| (d as char).to_digit(10).unwrap() as usize);
    let row: Parser<u8, Vec<usize>> = tree.repeat(1..);
    let forest: Parser<u8, Vec<Vec<usize>>> = list(row, sym(b'\n'));

    forest.parse(input.as_bytes()).unwrap()
}

fn is_visible(forest: &Vec<Vec<usize>>, tree: (usize, usize), depth: isize, width: isize) -> bool {
    if tree.0 == 0 || tree.1 == 0 || tree.0 as isize == depth as isize - 1 || tree.1 as isize == depth as isize - 1 {
        return true;
    }
    let height: usize = *forest.get(tree.0).unwrap().get(tree.1).unwrap();

    for d in DIR.iter() {
        let mut current: (isize, isize) = (tree.0 as isize + d.0, tree.1 as isize + d.1);
        let mut current_visible = true;
        while current.0 >= 0 && current.1 >= 0 && current.0 < depth && current.1 < width {
            let next: usize = *forest.get(current.0 as usize).unwrap().get(current.1 as usize).unwrap();
            if next >= height {
                current_visible = false;
                break;
            }
            current = (current.0 + d.0, current.1 + d.1);
        }
        if current_visible {
            return true;
        }
    }

    false
}

fn count_score(forest: &Vec<Vec<usize>>, tree: (usize, usize), depth: isize, width: isize) -> usize {
    let height: usize = *forest.get(tree.0).unwrap().get(tree.1).unwrap();

    let mut total_score: usize = 1;
    for d in DIR.iter() {
        let mut current: (isize, isize) = (tree.0 as isize + d.0, tree.1 as isize + d.1);
        let mut current_score: usize = 0;
        while current.0 >= 0 && current.1 >= 0 && current.0 < depth && current.1 < width {
            current_score += 1;
            let next: usize = *forest.get(current.0 as usize).unwrap().get(current.1 as usize).unwrap();
            if next >= height {
                break;
            }
            current = (current.0 + d.0, current.1 + d.1);
        }
        total_score *= current_score;
    }

    total_score
}

fn part_1(input: &String) -> usize {
    let forest = parse(input);
    let depth: usize = forest.len();
    let width: usize = forest[0].len();
    let mut visible: usize = 0;
    for (i, row) in forest.iter().enumerate() {
        for (j, _tree) in row.iter().enumerate() {
            if is_visible(&forest, (i, j), depth as isize, width as isize) {
                visible += 1;
            }
        }
    }

    visible
}

fn part_2(input: &String) -> usize {
    let forest = parse(input);
    let depth: usize = forest.len();
    let width: usize = forest[0].len();
    let mut max: usize = 0;
    for (i, row) in forest.iter().enumerate() {
        for (j, _tree) in row.iter().enumerate() {
            let next = count_score(&forest, (i, j), depth as isize, width as isize);
            if next > max {
                max = next;
            }
        }
    }

    max
}

fn main() {
    let input = read_string("inputs/day_08.txt");

    println!("Part 1 Results: {}", part_1(&input));
    println!("Part 2 Results: {}", part_2(&input));
}


#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_part_1() {
        let input = read_string("inputs/test/day_08.txt");

        assert_eq!(part_1(&input), 21);
    }

    #[test]
    fn test_part_2() {
        let input = read_string("inputs/test/day_08.txt");

        assert_eq!(part_2(&input), 8);
    }
}