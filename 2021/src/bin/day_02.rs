use aoc2021::get_input;
use itertools::Itertools;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum Direction {
    FORWARD,
    UP,
    DOWN
}

struct Instruction {
    direction: Direction,
    distance: usize
}

impl Instruction {
    pub fn new(line: &str) -> Self {
        let direction: Direction;
        let distance: usize;
        match line.split(' ').collect_tuple() {
            Some(("forward", dis)) => {
                direction = Direction::FORWARD;
                distance = dis.parse().unwrap();
            },
            Some(("up", dis)) => {
                direction = Direction::UP;
                distance = dis.parse().unwrap();
            },
            Some(("down", dis)) => {
                direction = Direction::DOWN;
                distance = dis.parse().unwrap();
            },
            _ => panic!("Unknown op code")
        }

        Self {
            direction,
            distance
        }
    }

    pub fn execute(&self, (x, y): (usize, usize)) -> (usize, usize) {
        match self.direction {
            Direction::FORWARD => (x + self.distance, y),
            Direction::UP => (x, y - self.distance),
            Direction::DOWN => (x, y + self.distance)
        }
    }

    pub fn execute_with_aim(&self, (x, y, aim): (usize, usize, usize)) -> (usize, usize, usize) {
        match self.direction {
            Direction::FORWARD => (x + self.distance, y + aim * self.distance, aim),
            Direction::UP => (x, y, aim - self.distance),
            Direction::DOWN => (x, y, aim + self.distance)
        }
    }
}

fn part_1(instructions: &[Instruction]) -> usize {
    let mut horizontal: usize = 0;
    let mut vertical: usize = 0;

    instructions.iter()
        .for_each(|instr| {
            let result = instr.execute((horizontal, vertical));
            horizontal = result.0;
            vertical = result.1;
        });

    horizontal * vertical
}

fn part_2(instructions: &[Instruction]) -> usize {
    let mut horizontal: usize = 0;
    let mut vertical: usize = 0;
    let mut aim: usize = 0;

    instructions.iter()
        .for_each(|instr| {
            let result = instr.execute_with_aim((horizontal, vertical, aim));
            horizontal = result.0;
            vertical = result.1;
            aim = result.2;
        });

    horizontal * vertical
}

fn main() {
    let input = get_input("inputs/day_02.txt", &Instruction::new);

    println!("Part 1 Results: {}", part_1(&input));
    println!("Part 2 Results: {}", part_2(&input));
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_part_1() {
        let input = get_input("inputs/test/day_02.txt", &Instruction::new);

        assert_eq!(part_1(&input), 150);
    }

    #[test]
    fn test_part_2() {
        let input = get_input("inputs/test/day_02.txt", &Instruction::new);

        assert_eq!(part_2(&input), 900);
    }
}