use std::collections::HashSet;
use std::ops::Add;

use aoc2022::{read_string, integer};
use pom::char_class::*;
use pom::parser::*;

struct Move {
    direction: Position,
    distance: usize
}

#[derive(PartialEq, Eq, Hash, Clone, Copy)]
struct Position {
    x: isize,
    y: isize
}

impl Add for Position {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y
        }
    }
}

fn parse(input: &String) -> Vec<Move> {
    let instruction: Parser<u8, Move> = (is_a(alpha).map(|c| c as char) - sym(b' ') + integer())
        .map(|x| {
            let dir: Position;
            match x.0 {
                'R' => { dir = Position { x: 1, y: 0 } },
                'L' => { dir = Position { x: -1, y: 0 } },
                'D' => { dir = Position { x: 0, y: -1 } },
                'U' => { dir = Position { x: 0, y: 1 } },
                _ => { dir = Position { x: 0, y: 0 } }
            }

            Move { direction: dir, distance: x.1 }
        });
    let instructions: Parser<u8, Vec<Move>> = list(instruction, sym(b'\n'));

    instructions.parse(input.as_bytes()).unwrap()
}

fn find_tail_position(tail: &Position, head: &Position) -> Position {
    let delta_x: isize = head.x - tail.x;
    let delta_y: isize = head.y - tail.y;
    let abs_dis: usize = (delta_x.abs() + delta_y.abs()) as usize;

    let mut x: isize = tail.x;
    let mut y: isize = tail.y;

    match delta_x {
        -2 => { x = tail.x - 1; },
        2 => { x = tail.x + 1; },
        -1 | 1 => { if abs_dis > 2 { x = tail.x + delta_x; } },
        _ => {}
    }

    match delta_y {
        -2 => { y = tail.y - 1; },
        2 => { y = tail.y + 1; },
        -1 | 1 => { if abs_dis > 2 { y = tail.y + delta_y; } },
        _ => {}
    }

    Position { x: x, y: y }
}

fn move_rope(moves: &Vec<Move>, knot_count: usize) -> usize {
    let mut head: Position = Position { x: 0, y: 0 };
    let mut knots: Vec<Position> = vec![Position {x: 0, y: 0}; knot_count];
    let mut tail_positions: HashSet<Position> = HashSet::new();
    tail_positions.insert(knots[knot_count - 1]);

    moves.iter()
        .for_each(|m| {
            (0..m.distance).for_each(|_| {
                head = head + m.direction;
                for i in 0..knot_count {
                    let old: &Position = knots.get(i).unwrap();
                    let previous: &Position = if i == 0 { &head } else { knots.get(i - 1).unwrap() };
                    knots[i] = find_tail_position(old, previous);
                }
                tail_positions.insert(knots[knot_count - 1]);
            })
        });

    tail_positions.len()
}

fn part_1(input: &String) -> usize {
    let moves: Vec<Move> = parse(input);
    move_rope(&moves, 1)
}

fn part_2(input: &String) -> usize {
    let moves: Vec<Move> = parse(input);
    move_rope(&moves, 9)
}

fn main() {
    let input = read_string("inputs/day_09.txt");

    println!("Part 1 Results: {}", part_1(&input));
    println!("Part 2 Results: {}", part_2(&input));
}


#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_part_1() {
        let input = read_string("inputs/test/day_09a.txt");

        assert_eq!(part_1(&input), 13);
    }

    #[test]
    fn test_part_2() {
        let input = read_string("inputs/test/day_09b.txt");

        assert_eq!(part_2(&input), 36);
    }
}