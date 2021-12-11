use aoc2021::{get_input, run};
use std::collections::HashMap;

fn read_input() -> Vec<u8> {
  get_input(6)
    .flat_map(|line| {
      line
        .split(',')
        .map(|v| v.parse::<u8>().expect("Couldn't parse number"))
        .collect::<Vec<_>>()
    })
    .collect()
}

fn calc(input: &[u8], iterations: u16) -> u64 {
  let mut fishes_age: HashMap<u8, u64> = HashMap::new();

  for age in 0..9 {
    fishes_age.insert(age, input.iter().filter(|&n| *n == age).count() as u64);
  }

  for _ in 0..iterations {
    let mut new_fishes_age: HashMap<u8, u64> = HashMap::new();

    for age in 0..9 {
      new_fishes_age.insert(age, 0);
    }

    for age in (0..9).rev() {
      if age == 0 {
        new_fishes_age.insert(8, fishes_age[&0]);
        new_fishes_age.insert(6, fishes_age[&0] + new_fishes_age[&6]);
      } else {
        new_fishes_age.insert(age - 1, fishes_age[&age]);
      }
    }

    fishes_age = new_fishes_age;
  }

  fishes_age.values().sum()
}

#[allow(clippy::ptr_arg)]
fn part_1(input: &Vec<u8>) -> u64 {
  calc(input, 80)
}

#[allow(clippy::ptr_arg)]
fn part_2(input: &Vec<u8>) -> u64 {
  calc(input, 256)
}

fn main() {
  run(read_input, part_1, part_2)
}
