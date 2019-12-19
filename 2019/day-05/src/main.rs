use std::fs;

enum ParameterMode {
  Position,
  Immediate
}

struct Machine {
  pc: usize,
  memory: Vec<i32>,
  input: Vec<i32>,
  output: Vec<i32>
}

impl Machine {
  pub fn new(memory: Vec<i32>, input: Vec<i32>) -> Self {
    Self {
      pc: 0,
      memory,
      input,
      output: Vec::new()
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

  pub fn execute(&mut self) {
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
  let program: Vec<i32> = contents
    .trim()
    .split(",")
    .map(|el| el.parse().unwrap())
    .collect();

  let mut machine = Machine::new(program.clone(), vec![1]);
  machine.execute();
  println!("Part 1 Result: {:?}", machine.output);

  let mut machine = Machine::new(program.clone(), vec![5]);
  machine.execute();
  println!("Part 2 Result: {:?}", machine.output);
}
