use std::collections::{HashMap, VecDeque};

fn read_input() -> Vec<String> {
  aoc2021::get_input(24).collect()
}

#[derive(Clone, Copy)]
enum ALUVar {
  W,
  X,
  Y,
  Z,
}

enum ALUParam {
  Var(ALUVar),
  Num(i64),
  Nothing,
}

#[derive(Copy, Clone, Debug)]
struct ALU {
  w: i64,
  x: i64,
  y: i64,
  z: i64,
}

impl ALU {
  fn new() -> ALU {
    ALU {
      w: 0,
      x: 0,
      y: 0,
      z: 0,
    }
  }

  fn run(&mut self, instructions: &[String], inputs: &mut VecDeque<i64>) {
    for instruction in instructions {
      self.parse_instruction(instruction, inputs);
    }
  }

  fn parse_instruction(&mut self, instruction: &str, inputs: &mut VecDeque<i64>) {
    let parts: Vec<&str> = instruction.split(' ').collect();
    let match_var = |val: &str| match val {
      "w" => ALUVar::W,
      "x" => ALUVar::X,
      "y" => ALUVar::Y,
      "z" => ALUVar::Z,
      _ => panic!("Not a variable"),
    };

    let op = parts[0];
    let var = match_var(parts[1]);
    let value = parts.get(2).map_or(ALUParam::Nothing, |val| {
      let maybe_number = val.parse::<i64>();
      match maybe_number {
        Ok(number) => ALUParam::Num(number),
        Err(_) => ALUParam::Var(match_var(val)),
      }
    });
    let input = inputs.pop_front();

    match op {
      "inp" => self.set(var, input.unwrap()),
      "add" => self.set(var, self.get(var) + self.unwrap(value)),
      "mul" => self.set(var, self.get(var) * self.unwrap(value)),
      "div" => self.set(var, self.get(var) / self.unwrap(value)),
      "mod" => self.set(var, self.get(var) % self.unwrap(value)),
      "eql" => self.set(var, (self.get(var) == self.unwrap(value)) as i64),
      op => panic!("operation {} not supported", op),
    }
  }

  fn unwrap(&self, value: ALUParam) -> i64 {
    match value {
      ALUParam::Num(val) => val,
      ALUParam::Var(var) => self.get(var),
      ALUParam::Nothing => panic!("Illegal"),
    }
  }

  fn get(&self, var: ALUVar) -> i64 {
    match var {
      ALUVar::W => self.w,
      ALUVar::X => self.x,
      ALUVar::Y => self.y,
      ALUVar::Z => self.z,
    }
  }

  fn set(&mut self, var: ALUVar, value: i64) {
    match var {
      ALUVar::W => self.w = value,
      ALUVar::X => self.x = value,
      ALUVar::Y => self.y = value,
      ALUVar::Z => self.z = value,
    }
  }
}

fn find_serial_numbers(monad: &[String]) -> impl Iterator<Item = u64> {
  let mut steps = vec![];
  let mut critical_steps = vec![];
  {
    let mut step = vec![];
    for line in monad {
      if line.contains("inp w") && !step.is_empty() {
        steps.push(step);
        step = vec![];
      }
      if line.contains("div z 26") {
        critical_steps.push(steps.len());
      }
      step.push(line.to_string());
    }
    steps.push(step);
  }

  let mut valid_serial_numbers: HashMap<String, ALU> = HashMap::new();

  for i in 1..=9 {
    let mut alu = ALU::new();
    let mut queue = VecDeque::from([i]);
    alu.run(&steps[0], &mut queue);
    valid_serial_numbers.insert(i.to_string(), alu);
  }

  for (idx, step) in steps.iter().enumerate().skip(1) {
    let (_, min_alu) = valid_serial_numbers
      .iter()
      .min_by(|(_, alu_1), (_, alu_2)| alu_1.z.cmp(&alu_2.z))
      .unwrap();

    let mut new_valid_serial_numbers: HashMap<String, ALU> = HashMap::new();

    for (num, alu) in valid_serial_numbers.iter() {
      for i in 1..=9 {
        let new_num = format!("{}{}", num, i);
        let mut queue = VecDeque::from([i]);
        let mut new_alu = alu.clone();
        new_alu.run(step, &mut queue);
        new_valid_serial_numbers.insert(new_num, new_alu);
      }
    }

    if critical_steps.contains(&idx) {
      new_valid_serial_numbers.retain(|_, alu| alu.z < min_alu.z);
    }

    valid_serial_numbers = new_valid_serial_numbers;
  }

  valid_serial_numbers
    .into_iter()
    .flat_map(|(serial, _)| serial.parse::<u64>())
}

#[allow(clippy::ptr_arg)]
fn part_1(input: &Vec<String>) -> u64 {
  find_serial_numbers(input).max().unwrap()
}

#[allow(clippy::ptr_arg)]
fn part_2(input: &Vec<String>) -> u64 {
  find_serial_numbers(input).min().unwrap()
}

fn main() {
  aoc2021::run(read_input, part_1, part_2)
}
