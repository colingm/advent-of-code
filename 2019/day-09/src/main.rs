use std::fs;

#[derive(Debug)]
enum ParameterMode {
  Position,
  Immediate,
  Relative
}

struct Machine {
  pc: usize,
  memory: Vec<i64>,
  input: Vec<i64>,
  output: Vec<i64>,
  relative_offset: i64,
  feedback_mode: bool,
  halted: bool
}

impl Machine {
  pub fn new(memory: Vec<i64>, input: Vec<i64>, feedback_mode: bool) -> Self {

    let mut full_memory = memory.clone();
    let mut extra_memory: Vec<i64> = vec![0; 10000];
    full_memory.append(&mut extra_memory);
    Self {
      pc: 0,
      memory: full_memory,
      input,
      output: Vec::new(),
      relative_offset: 0,
      feedback_mode: feedback_mode,
      halted: false
    }
  }

  fn get_opcode(&self) -> i64 {
    self.memory[self.pc] % 100
  }

  fn get_param_mode(&self, offset: usize) -> ParameterMode {
    match (self.memory[self.pc] as usize / (10_usize.pow(offset as u32 + 1))) % 10 {
      2 => ParameterMode::Relative,
      1 => ParameterMode::Immediate,
      _ => ParameterMode::Position
    }
  }

  fn get_param_value(&self, offset: usize) -> i64 {
    let address = self.get_address(offset);

    self.memory[address]
  }

  fn get_address(&self, offset: usize) -> usize {
    let memory = &self.memory;
    let param_mode = self.get_param_mode(offset);
    let mut address = self.pc + offset;

    match param_mode {
      ParameterMode::Position => {
        address = memory[address] as usize;
      },
      ParameterMode::Relative => {
        address = (memory[address] as i64 + self.relative_offset) as usize;
      },
      _ => {}
    }

    address
  }

  pub fn _add_input(&mut self, input_value: i64) {
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
          self.memory[address] = (p1 < p2) as i64;

          4
        },
        // equals
        8 => {
          let (p1, p2) = (self.get_param_value(1), self.get_param_value(2));
          let address = self.get_address(3);
          self.memory[address] = (p1 == p2) as i64;

          4
        },
        // adjust relative offset
        9 => {
          let p1 = self.get_param_value(1);
          self.relative_offset += p1;

          2
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

fn main() {
  let contents = fs::read_to_string("input.txt")
    .expect("Something went wrong reading the file");
  let program: Vec<i64> = contents
    .trim()
    .split(",")
    .map(|el| el.parse().unwrap())
    .collect();

  let mut machine = Machine::new(program.clone(), vec![1], false);
  machine.execute();
  println!("Part 1 Result: {:?}", machine.output);

  let mut machine = Machine::new(program.clone(), vec![2], false);
  machine.execute();
  println!("Part 2 Result: {:?}", machine.output);
}

#[cfg(test)]
mod test {
  use super::*;

  #[test]
  fn test_1() {
    let test = "109,1,204,-1,1001,100,1,100,1008,100,16,101,1006,101,0,99";
    let program: Vec<i64> = test
      .split(",")
      .map(|el| el.parse().unwrap())
      .collect();

    let mut machine = Machine::new(program.clone(), Vec::new(), false);
    machine.execute();
    let output: Vec<String> = machine.output.iter().map(|e| e.to_string()).collect();
    assert_eq!(output.join(","), test);
  }

  #[test]
  fn test_2() {
    let program: Vec<i64> = "1102,34915192,34915192,7,4,7,99,0"
      .split(",")
      .map(|el| el.parse().unwrap())
      .collect();

    let mut machine = Machine::new(program.clone(), Vec::new(), false);
    machine.execute();
    let output: Vec<String> = machine.output.iter().map(|e| e.to_string()).collect();
    assert_eq!(output.join("").len(), 16);
  }

  #[test]
  fn test_3() {
    let program: Vec<i64> = "104,1125899906842624,99"
      .split(",")
      .map(|el| el.parse().unwrap())
      .collect();

    let mut machine = Machine::new(program.clone(), Vec::new(), false);
    machine.execute();
    let output: Vec<String> = machine.output.iter().map(|e| e.to_string()).collect();
    assert_eq!(output.join(""), "1125899906842624".to_string());
  }
}
