use aoc2021::get_input;
use regex::Regex;
use std::collections::HashMap;
use lazy_static::lazy_static;

lazy_static! {
    pub static ref LINE_RE: Regex = Regex::new(r"(?P<x1>\d+),(?P<y1>\d+) -> (?P<x2>\d+),(?P<y2>\d+)").unwrap();
}

#[derive(Debug)]
struct Point {
    pub x: usize,
    pub y: usize
}

#[derive(Debug)]
struct Line {
    start: Point,
    end: Point
}

impl Line {
    pub fn new(coordinates: &str) -> Self {
        let caps = LINE_RE.captures(coordinates).unwrap();

        let start = Point { x: caps["x1"].parse().unwrap(), y: caps["y1"].parse().unwrap() };
        let end = Point { x: caps["x2"].parse().unwrap(), y: caps["y2"].parse().unwrap() };

        Self {
            start,
            end
        }
    }

    pub fn is_horizontal(&self) -> bool {
        self.start.x == self.end.x
    }

    pub fn is_vertical(&self) -> bool {
        self.start.y == self.end.y
    }

    pub fn is_not_diagonal(&self) -> bool {
        self.is_horizontal() || self.is_vertical()
    }

    pub fn get_points(&self) -> Vec<Point> {
        let x_inc: isize = match (self.start.x > self.end.x, self.start.x < self.end.x) {
            (true, _) => -1,
            (_, true) => 1,
            _ => 0
        };

        let y_inc: isize = match (self.start.y > self.end.y, self.start.y < self.end.y) {
            (true, _) => -1,
            (_, true) => 1,
            _ => 0
        };

        let mut points: Vec<Point> = Vec::new();

        let mut x = self.start.x as isize;
        let mut y = self.start.y as isize;

        while x as usize != self.end.x || y as usize != self.end.y {
            points.push(Point { x: x as usize, y: y as usize });
            x += x_inc;
            y += y_inc;
        }

        points.push(Point { x: x as usize, y: y as usize });

        points
    }
}

#[derive(Debug)]
struct VentMap {
    vents: HashMap<usize, HashMap<usize, usize>>
}

impl VentMap {
    pub fn new() -> Self {
        Self {
            vents: HashMap::new()
        }
    }

    pub fn apply_line(&mut self, line: &Line, include_diagonals: bool) {
        if include_diagonals || line.is_not_diagonal() {
            let points = line.get_points();

            for point in points.iter() {
                *((*self.vents.entry(point.x).or_insert(HashMap::new())).entry(point.y).or_insert(0)) += 1;
            }
        }
    }

    pub fn count_dangerous_vents(&self) -> usize {
        self.vents.iter()
            .map(|(_, row)| {
                let row_sum: usize = row.iter()
                    .filter(|(_, column)| {
                        (**column) > 1
                    })
                    .count();

                row_sum
            })
            .sum()
    }
}

fn part_1(lines: &[Line]) -> usize {
    let mut vents = VentMap::new();
    for line in lines.iter() {
        vents.apply_line(&line, false);
    }

    vents.count_dangerous_vents()
}

fn part_2(lines: &[Line]) -> usize {
    let mut vents = VentMap::new();
    for line in lines.iter() {
        vents.apply_line(&line, true);
    }

    vents.count_dangerous_vents()
}

fn main() {
    let input = get_input("inputs/day_05.txt", &Line::new);

    println!("Part 1 Results: {}", part_1(&input));
    println!("Part 2 Results: {}", part_2(&input));
}


#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_part_1() {
        let input = get_input("inputs/test/day_05.txt", &Line::new);

        assert_eq!(part_1(&input), 5);
    }

    #[test]
    fn test_part_2() {
        let input = get_input("inputs/test/day_05.txt", &Line::new);

        assert_eq!(part_2(&input), 12);
    }

    #[test]
    fn test_part_1_answer() {
        let input = get_input("inputs/day_05.txt", &Line::new);

        assert_eq!(part_1(&input), 5124);
    }

    #[test]
    fn test_part_2_answer() {
        let input = get_input("inputs/day_05.txt", &Line::new);

        assert_eq!(part_2(&input), 19771);
    }
}