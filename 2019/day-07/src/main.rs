extern crate itertools;
use std::fs;
use itertools::Itertools;

enum ParameterMode {
  Position,
  Immediate
}

struct Machine {
  pc: usize,
  memory: Vec<i32>,
  input: Vec<i32>,
  output: Vec<i32>,
  feedback_mode: bool,
  halted: bool
}

impl Machine {
  pub fn new(memory: Vec<i32>, input: Vec<i32>, feedback_mode: bool) -> Self {
    Self {
      pc: 0,
      memory,
      input,
      output: Vec::new(),
      feedback_mode: feedback_mode,
      halted: false
    }
  }

  fn get_opcode(&self) -> i32 {
    self.memory[self.pc] % 100
  }

  fn get_param_mode(&self, offset: usize) -> ParameterMode {
    match (self.memory[self.pc] as usize / (10_usize.pow(offset as u32 + 1))) % 10 {
      1 => ParameterMode::Immediate,
      _ => ParameterMode::Position
    }
  }

  fn get_param_value(&self, offset: usize) -> i32 {
    let memory = &self.memory;
    let param_mode = self.get_param_mode(offset);
    let mut address = self.pc + offset;

    if let ParameterMode::Position = param_mode {
      address = memory[address] as usize;
    }

    memory[address]
  }

  fn get_address(&self, offset: usize) -> usize {
    self.memory[self.pc + offset] as usize
  }

  pub fn add_input(&mut self, input_value: i32) {
    self.input.push(input_value);
  }

  pub fn execute(&mut self) {
    if self.halted {
      panic!("Cannot run a halted machine");
    }

    loop {
      let pc = self.pc;
      let opcode = self.get_opcode();

      let step = match opcode {
        // sum
        1 => {
          let (p1, p2) = (self.get_param_value(1), self.get_param_value(2));
          let address = self.get_address(3);
          self.memory[address] = p1 + p2;

          4
        },
        // mul
        2 => {
          let (p1, p2) = (self.get_param_value(1), self.get_param_value(2));
          let address = self.get_address(3);
          self.memory[address] = p1 * p2;

          4
        },
        // store
        3 => {
          let address = self.get_address(1);
          self.memory[address] = self.input.remove(0);

          2
        },
        // read
        4 => {
          let value = self.get_param_value(1);
          self.output.push(value);

          if self.feedback_mode {
            self.pc += 2;
            break;
          }

          2
        },
        // jump-if-true
        5 => {
          let value = self.get_param_value(1);
          if value != 0 {
            let address = self.get_param_value(2) as usize;
            self.pc = address;
            0
          } else {
            3
          }
        },
        // jump-if-false
        6 => {
          let value = self.get_param_value(1);
          if value == 0 {
            let address = self.get_param_value(2) as usize;
            self.pc = address;
            0
          } else {
            3
          }
        },
        // less than
        7 => {
          let (p1, p2) = (self.get_param_value(1), self.get_param_value(2));
          let address = self.get_address(3);
          self.memory[address] = (p1 < p2) as i32;

          4
        },
        // equals
        8 => {
          let (p1, p2) = (self.get_param_value(1), self.get_param_value(2));
          let address = self.get_address(3);
          self.memory[address] = (p1 == p2) as i32;

          4
        },
        // exit
        99 => {
          self.halted = true;
          break;
        },
        _ => {
          panic!("Invalid Opcode: {} @ {}", opcode, pc)
        }
      };

      self.pc += step;
    }
  }
}

fn calculate_max_from_sequence(program: &Vec<i32>) -> i32 {
  let mut max = 0;
  let combos = vec![0,1,2,3,4];
  for combo in combos.iter().permutations(5) {
    let mut result = 0;
    for i in combo.iter() {
      let mut machine = Machine::new(program.clone(), vec![(*i).clone(), result], false);
      machine.execute();
      result = machine.output.last().unwrap().clone();
    }

    if result > max {
      max = result;
    }
  }

  max
}

fn calculate_max_with_feedback(program: &Vec<i32>) -> i32 {
  let mut max = 0;
  let combos = vec![5,6,7,8,9];
  for combo in combos.iter().permutations(5) {
    let mut result = 0;
    let mut done = false;
    let mut index = 0;
    let mut machines: Vec<Machine> = Vec::new();
    while !done {
      let mut new_machine = false;
      if machines.len() == index {
        machines.push(Machine::new(program.clone(), vec![*combo[index], result], true));
        new_machine = true;
      }

      let machine = machines.get_mut(index).unwrap();
      if !new_machine {
        machine.add_input(result);
      }
      machine.execute();
      result = machine.output.last().unwrap().clone();

      if machine.halted && index == 4 {
        done = true;
        if result > max {
          max = result;
        }
      }
      index += 1;
      if index > 4 {
        index = 0;
      }
    }
  }

  max
}

fn main() {
  let contents = fs::read_to_string("input.txt")
    .expect("Something went wrong reading the file");
  let program: Vec<i32> = contents
    .trim()
    .split(",")
    .map(|el| el.parse().unwrap())
    .collect();

  println!("Part 1 Result: {:?}", calculate_max_from_sequence(&program.clone()));
  println!("Part 2 Result: {:?}", calculate_max_with_feedback(&program.clone()));

}

#[cfg(test)]
mod test {
  use super::*;

  #[test]
  fn test_1() {
    let program: Vec<i32> = "3,15,3,16,1002,16,10,16,1,16,15,15,4,15,99,0,0"
      .split(",")
      .map(|el| el.parse().unwrap())
      .collect();

    assert_eq!(calculate_max_from_sequence(&program), 43210);
  }

  #[test]
  fn test_2() {
    let program: Vec<i32> = "3,23,3,24,1002,24,10,24,1002,23,-1,23,101,5,23,23,1,24,23,23,4,23,99,0,0"
      .split(",")
      .map(|el| el.parse().unwrap())
      .collect();

    assert_eq!(calculate_max_from_sequence(&program), 54321);
  }

  #[test]
  fn test_3() {
    let program: Vec<i32> = "3,31,3,32,1002,32,10,32,1001,31,-2,31,1007,31,0,33,1002,33,7,33,1,33,31,31,1,32,31,31,4,31,99,0,0,0"
      .split(",")
      .map(|el| el.parse().unwrap())
      .collect();

    assert_eq!(calculate_max_from_sequence(&program), 65210);
  }

  #[test]
  fn test_4() {
    let program: Vec<i32> = "3,26,1001,26,-4,26,3,27,1002,27,2,27,1,27,26,27,4,27,1001,28,-1,28,1005,28,6,99,0,0,5"
      .split(",")
      .map(|el| el.parse().unwrap())
      .collect();

    assert_eq!(calculate_max_with_feedback(&program), 139629729);
  }

  #[test]
  fn test_5() {
    let program: Vec<i32> = "3,52,1001,52,-5,52,3,53,1,52,56,54,1007,54,5,55,1005,55,26,1001,54,-5,54,1105,1,12,1,53,54,53,1008,54,0,55,1001,55,1,55,2,53,55,53,4,53,1001,56,-1,56,1005,56,6,99,0,0,0,0,10"
      .split(",")
      .map(|el| el.parse().unwrap())
      .collect();

    assert_eq!(calculate_max_with_feedback(&program), 18216);
  }
}
