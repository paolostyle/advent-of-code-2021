use aoc2021::get_input;
use std::io::*;

struct Instruction {
  direction: String,
  units: u32,
}

fn read_input() -> Vec<Instruction> {
  get_input(2)
    .lines()
    .map(|line| {
      let read_line = line.expect("could not read line");
      let mut iter = read_line.split_whitespace();
      let direction = String::from(iter.next().unwrap());
      let units = iter
        .next()
        .unwrap()
        .parse::<u32>()
        .expect("Could not parse units");

      Instruction { direction, units }
    })
    .collect()
}

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
  let input = read_input();
  println!("Part 1: {}", part_1(&input));
  println!("Part 2: {}", part_2(&input));
}
