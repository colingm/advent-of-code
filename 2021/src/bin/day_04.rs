use aoc2021::{get_input, parse_string};
use std::collections::VecDeque;

#[derive(Debug)]
struct Cell {
    pub value: usize,
    pub marked: bool
}

#[derive(Debug)]
struct Board {
    pub rows: Vec<Vec<Cell>>
}

impl Board {
    pub fn new(lines: &Vec<String>) -> Self {
        let rows: Vec<Vec<Cell>> = lines.iter()
            .map(|line| line.split_whitespace()
                .map(|val| Cell { value: val.parse().unwrap(), marked: false })
                .collect()
            )
            .collect();

        Self {
            rows
        }
    }

    pub fn check_for_win(&self) -> bool {
        let row_win = self.rows.iter()
            .any(|row| row.iter().all(|c| c.marked));

        if row_win {
            return true
        }

        let column_win = (0..5).collect::<Vec<usize>>().iter().any(|i| self.rows.iter().all(|row| row[*i].marked));

        if column_win {
            return true
        }

        false
    }

    pub fn mark_number(&mut self, number: usize) {
        for row in self.rows.iter_mut() {
            for cell in row.iter_mut() {
                if cell.value == number {
                    cell.marked = true;
                }
            }
        }
    }

    pub fn count_score(&self) -> usize {
        let mut sum: usize = 0;
        for row in &self.rows {
            for cell in row.iter() {
                if !cell.marked {
                    sum += cell.value;
                }
            }
        }

        sum
    }
}

struct Bingo {
    numbers: VecDeque<usize>,
    pub boards: Vec<Board>
}

impl Bingo {
    pub fn new(lines: &Vec<String>) -> Self {
        let mut current = lines.to_vec();
        let numbers: VecDeque<usize>;
        let mut boards: Vec<Board> = Vec::new();
        numbers = current.remove(0).split(",")
            .map(|number| number.parse().unwrap())
            .collect();

        current.retain(|x| !x.is_empty());

        while current.len() > 0 {
            let remaining = current.split_off(5);
            boards.push(Board::new(&current));
            current = remaining;
        }

        Self {
            numbers,
            boards
        }
    }

    fn check_for_win(&self) -> isize {
        let win = self.boards.iter()
            .enumerate()
            .find(|(_index, board)| board.check_for_win());

        match win {
            Some((index, _board)) => index as isize,
            _ => -1
        }
    }

    pub fn find_winning_board(&mut self) -> (isize, usize) {
        let mut winning_board: isize = -1;
        let mut number: usize = 0;

        while winning_board == -1 {
            number = self.numbers.pop_front().unwrap();

            for board in self.boards.iter_mut() {
                board.mark_number(number);
            }

            winning_board = self.check_for_win();
        }

        (winning_board, number)
    }

    pub fn find_losing_board_score(&mut self) -> usize {
        let mut number: usize = 0;

        while self.boards.len() > 1 {
            number = self.numbers.pop_front().unwrap();

            for board in self.boards.iter_mut() {
                board.mark_number(number);
            }

            self.boards.retain(|board| !board.check_for_win());
        }

        while !self.boards[0].check_for_win() {
            number = self.numbers.pop_front().unwrap();

            self.boards[0].mark_number(number);
        }

        self.boards[0].count_score() * number
    }
}

fn part_1(lines: &Vec<String>) -> usize {
    let mut bingo = Bingo::new(lines);

    let results = bingo.find_winning_board();

    match results.0 {
        -1 => 0,
        _ => bingo.boards[results.0 as usize].count_score() * results.1
    }
}

fn part_2(lines: &Vec<String>) -> usize {
    let mut bingo = Bingo::new(lines);

    bingo.find_losing_board_score()
}

fn main() {
    let input = get_input("inputs/day_04.txt", &parse_string);

    println!("Part 1 Results: {}", part_1(&input));
    println!("Part 2 Results: {}", part_2(&input));
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_part_1() {
        let input = get_input("inputs/test/day_04.txt", &parse_string);

        assert_eq!(part_1(&input), 4512);
    }

    #[test]
    fn test_part_2() {
        let input = get_input("inputs/test/day_04.txt", &parse_string);

        assert_eq!(part_2(&input), 1924);
    }
}