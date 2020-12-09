use std::fs;
use itertools::Itertools;

#[derive(Clone, Copy, Debug)]
enum Instruction {
    NOP(isize),
    ACC(isize),
    JMP(isize)
}

impl Instruction {
    pub fn new(line: &str) -> Self {
        match line.split(' ').collect_tuple() {
            Some(("nop", x)) => Self::NOP(x.parse().unwrap()),
            Some(("acc", x)) => Self::ACC(x.parse().unwrap()),
            Some(("jmp", x)) => Self::JMP(x.parse().unwrap()),
            _ => panic!("Unknown op code")
        }
    }

    pub fn execute(self, permute: bool) -> (isize, isize) {
        match self {
            Self::NOP(x) => if permute { (x, 0) } else { (1, 0) },
            Self::ACC(x) => (1, x),
            Self::JMP(x) => if permute { (1, 0) } else { (x, 0) }
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum Status {
    Running,
    Looping,
    Finished
}

struct Program<'a> {
    instructions: &'a Vec<Instruction>,
    status: Status,
    pc: usize,
    acc: isize,
    executed: Vec<bool>,
    permutation: Option<usize>
}

impl<'a> Program<'a> {
    pub fn new(instructions: &'a Vec<Instruction>, permutation: Option<usize>) -> Self {
        Program {
            instructions,
            status: Status::Running,
            pc: 0,
            acc: 0,
            executed: (0..instructions.len()).map(|_| false).collect(),
            permutation
        }
    }

    pub fn execute(&mut self) {
        let permute = self.permutation == Some(self.pc);
        self.executed[self.pc] = true;
        let (pc_inc, acc_inc) = self.instructions[self.pc].execute(permute);
        self.pc = (self.pc as isize + pc_inc) as usize;
        self.acc += acc_inc;
    }

    pub fn next(&mut self) {
        if self.pc >= self.instructions.len() {
            self.status = Status::Finished;
        } else if self.executed[self.pc] {
            self.status = Status::Looping;
        } else {
            self.execute();
        }
    }

    pub fn run(&mut self) -> (isize, Status) {
        while self.status != Status::Looping && self.status != Status::Finished {
            self.next();
        }

        (self.acc, self.status)
    }
}

struct ProgramPermutations<'a> {
    instructions: &'a Vec<Instruction>,
    line: usize,
}

impl<'a> Iterator for ProgramPermutations<'a> {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            let permute_possible = match self.instructions[self.line] {
                Instruction::JMP(_) | Instruction::NOP(_) => true,
                _ => false
            };
            if permute_possible {
                let permutation = Some(self.line);
                self.line += 1;
                return permutation;
            } else {
                if self.line >= self.instructions.len() {
                    return None;
                } else {
                    self.line += 1;
                }
            }
        }
    }
}

fn part1(instructions: &Vec<Instruction>) -> isize {
    Program::new(instructions, None).run().0
}

fn part2(instructions: &Vec<Instruction>) -> isize {
    ProgramPermutations {
        instructions,
        line: 0
    }
    .find_map(|permutation| {
        let (acc, status) = Program::new(instructions, Some(permutation)).run();
        match status {
            Status::Finished => Some(acc),
            _ => None
        }
    })
    .unwrap()
}

fn get_input(filename: &str) -> Vec<Instruction> {
    let contents = fs::read_to_string(filename)
        .expect("Something went wrong reading the file");

    contents
        .lines()
        .map(Instruction::new)
        .collect()
}

fn main() {
    let instructions = get_input("input.txt");

    println!("Part 1 Results: {}", part1(&instructions));
    println!("Part 2 Results: {}", part2(&instructions));
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_find_loop() {
        let instructions = get_input("test.txt");

        assert_eq!(part1(&instructions), 5);
    }

    #[test]
    fn test_fix_program() {
        let instructions = get_input("test.txt");

        assert_eq!(part2(&instructions), 8);
    }
}
