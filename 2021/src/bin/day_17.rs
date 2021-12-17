use aoc2021::{get_input, parse_string};
use regex::Regex;
use lazy_static::lazy_static;

lazy_static! {
    pub static ref AREA_RE: Regex = Regex::new(r"target area: x=(?P<x1>-?\d+)\.\.(?P<x2>-?\d+), y=(?P<y1>-?\d+)\.\.(?P<y2>-?\d+)").unwrap();
}

fn find_x_start(x_min: isize) -> isize {
    let under_square_root: isize = 8 * x_min + 1;
    let root: f64 = (under_square_root as f64).sqrt();
    let rounded: isize = root.round() as isize;

    ((rounded - 1) / 2) - 1
}

fn will_enter_target(x_vel: isize, y_vel: isize, x_min: isize, x_max: isize, y_min: isize, y_max: isize) -> bool {
    let mut x: isize = 0;
    let mut y: isize = 0;
    let mut x_velocity: isize = x_vel;
    let mut y_velocity: isize = y_vel;

    while x <= x_max && y >= y_min {
        x += x_velocity;
        y += y_velocity;

        if x >= x_min && x <= x_max && y >= y_min && y <= y_max {
            return true;
        }

        if x_velocity > 0 {
            x_velocity -= 1;
        }
        y_velocity -= 1;
    }

    false
}

fn part_1(rows: &Vec<String>) -> usize {
    let area_string: &String = &rows[0];
    let cap = AREA_RE.captures(area_string).unwrap();

    let y_min: usize = cap["y1"].parse::<isize>().unwrap().abs() as usize;

    (y_min - 1) * y_min / 2
}

fn part_2(rows: &Vec<String>) -> usize {
    let area_string: &String = &rows[0];
    let cap = AREA_RE.captures(area_string).unwrap();

    let y_min: isize = cap["y1"].parse().unwrap();
    let y_max: isize = cap["y2"].parse().unwrap();
    let x_min: isize = cap["x1"].parse().unwrap();
    let x_max: isize = cap["x2"].parse().unwrap();

    let x_start: isize = find_x_start(x_min);

    let mut matches: usize = 0;
    for y in y_min..=(y_min.abs() - 1) {
        for x in x_start..=(x_max) {
            if will_enter_target(x, y, x_min, x_max, y_min, y_max) {
                matches += 1;
            }
        }
    }

    matches
}

fn main() {
    let input: Vec<String> = get_input("inputs/day_17.txt", &parse_string);

    println!("Part 1 Results: {}", part_1(&input));
    println!("Part 2 Results: {}", part_2(&input));
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_part_1() {
        let input = get_input("inputs/test/day_17.txt", &parse_string);

        assert_eq!(part_1(&input), 45);
    }

    #[test]
    fn test_part_1_answer() {
        let input = get_input("inputs/day_17.txt", &parse_string);

        assert_eq!(part_1(&input), 11781);
    }

    #[test]
    fn test_part_2() {
        let input = get_input("inputs/test/day_17.txt", &parse_string);

        assert_eq!(part_2(&input), 112);
    }

    #[test]
    fn test_part_2_answer() {
        let input = get_input("inputs/day_17.txt", &parse_string);

        assert_eq!(part_2(&input), 4531);
    }
}