use aoc2021::{get_input, run};

struct Instruction {
  direction: String,
  units: u32,
}

fn read_input() -> Vec<Instruction> {
  get_input(2)
    .map(|line| {
      let mut iter = line.split_whitespace();
      let direction = String::from(iter.next().unwrap());
      let units = iter
        .next()
        .unwrap()
        .parse()
        .expect("Could not parse units");

      Instruction { direction, units }
    })
    .collect()
}

#[allow(clippy::ptr_arg)]
fn part_1(input: &Vec<Instruction>) -> u32 {
  let mut horizontal: u32 = 0;
  let mut depth: u32 = 0;

  for Instruction { direction, units } in input {
    match direction.as_str() {
      "down" => depth += units,
      "up" => depth -= units,
      "forward" => horizontal += units,
      _ => panic!("unknown direction"),
    }
  }

  horizontal * depth
}

#[allow(clippy::ptr_arg)]
fn part_2(input: &Vec<Instruction>) -> u32 {
  let mut horizontal: u32 = 0;
  let mut depth: u32 = 0;
  let mut aim: u32 = 0;

  for Instruction { direction, units } in input {
    match direction.as_str() {
      "down" => aim += units,
      "up" => aim -= units,
      "forward" => {
        horizontal += units;
        depth += aim * units;
      }
      _ => panic!("unknown direction"),
    }
  }

  horizontal * depth
}

fn main() {
  run(read_input, part_1, part_2)
}
