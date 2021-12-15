use aoc2021::{get_input, parse_string};
use regex::Regex;
use lazy_static::lazy_static;

lazy_static! {
    pub static ref POINT_RE: Regex = Regex::new(r"(?P<x>\d+),(?P<y>\d+)").unwrap();
    pub static ref FOLD_RE: Regex = Regex::new(r"fold along (?P<axis>\w)=(?P<line>\d+)").unwrap();
}

#[derive(Eq, Hash, PartialEq, Copy, Clone)]
struct Point {
    x: usize,
    y: usize
}

#[derive(Debug)]
struct Fold {
    vertical: bool,
    line: usize
}

#[derive(Debug)]
struct Manual {
    dots: Vec<Vec<bool>>,
    folds: Vec<Fold>,
    current_fold: usize
}

impl Manual {
    fn new(rows: &Vec<String>) -> Self {
        let mut max_x: usize = 0;
        let mut max_y: usize = 0;
        let mut points: Vec<Point> = Vec::new();
        let mut folds: Vec<Fold> = Vec::new();

        for row in rows {
            let point_cap = POINT_RE.captures(row);
            if point_cap.is_some() {
                let point = point_cap.unwrap();
                let x: usize = point["x"].parse().unwrap();
                let y: usize = point["y"].parse().unwrap();
                if x > max_x {
                    max_x = x;
                }
                if y > max_y {
                    max_y = y;
                }
                points.push(Point { x, y });
            } else {
                let fold_cap = FOLD_RE.captures(row);
                if fold_cap.is_some() {
                    let fold = fold_cap.unwrap();
                    let vertical: bool = fold["axis"] == *"x";
                    let line: usize = fold["line"].parse().unwrap();
                    folds.push(Fold { vertical, line });
                }
            }
        }

        let mut dots: Vec<Vec<bool>> = Vec::new();
        for _ in 0..=max_y {
            dots.push(vec![false; max_x + 1]);
        }

        for point in points {
            dots[point.y][point.x] = true;
        }

        Self {
            dots,
            folds,
            current_fold: 0
        }
    }

    fn count_dots(&self) -> usize {
        self.dots.iter()
            .map(|row| row.iter().filter(|val| **val).count())
            .sum()
    }

    fn fold_page(&mut self) {
        let fold = &self.folds[self.current_fold];
        let rows = self.dots.len() - 1;
        let columns = self.dots[0].len() - 1;

        if fold.vertical {
            let diff = columns - fold.line;
            for row in &mut self.dots {
                for x in fold.line..columns {
                    let old = row.pop().unwrap();
                    row[x - diff] |= old;
                }
                row.pop();
            }
        } else {
            let diff = rows - fold.line;
            for y in fold.line..rows {
                let old_line = self.dots.pop().unwrap();
                for (x, val) in old_line.iter().enumerate() {
                    self.dots[y - diff][x] |= val;
                }
            }
            self.dots.pop();
        }

        self.current_fold += 1;
    }

    fn complete_folds(&mut self) {
        while self.current_fold < self.folds.len() {
            self.fold_page();
        }
    }

    fn print(&self) {
        for row in &self.dots {
            let line: String = row.iter().map(|val| match val { true => '#', false => '.'}).collect();
            println!("{}", line);
        }
    }
}


fn part_1(rows: &Vec<String>) -> usize {
    let mut manual: Manual = Manual::new(rows);

    manual.fold_page();
    manual.count_dots()
}

fn part_2(rows: &Vec<String>) -> usize {
    let mut manual: Manual = Manual::new(rows);

    manual.complete_folds();
    manual.print();
    0
}

fn main() {
    let input: Vec<String> = get_input("inputs/day_13.txt", &parse_string);

    println!("Part 1 Results: {}", part_1(&input));
    println!("Part 2 Results: {}", part_2(&input));
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_part_1() {
        let input = get_input("inputs/test/day_13.txt", &parse_string);

        assert_eq!(part_1(&input), 17);
    }

    #[test]
    fn test_part_1_answer() {
        let input = get_input("inputs/day_13.txt", &parse_string);

        assert_eq!(part_1(&input), 655);
    }

    // #[test]
    // fn test_part_2() {
    //     let input = get_input("inputs/test/day_13.txt", &parse_string);

    //     assert_eq!(part_2(&input), 195);
    // }

    // #[test]
    // fn test_part_2_answer() {
    //     let input = get_input("inputs/day_13.txt", &parse_string);

    //     assert_eq!(part_2(&input), 210);
    // }
}