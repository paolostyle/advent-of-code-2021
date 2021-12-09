use aoc2021::get_input;
use std::{io::BufRead, time::Instant};

fn read_input() -> Vec<i32> {
  get_input(7)
    .lines()
    .flat_map(|line| {
      line
        .expect("Couldn't read line")
        .split(",")
        .map(|v| v.parse::<i32>().expect("Couldn't parse number"))
        .collect::<Vec<_>>()
    })
    .collect()
}

fn part_1(input: &Vec<i32>) -> i32 {
  let max_val = *input.iter().max().unwrap();
  let mut min_result = i32::MAX;

  // this is kinda brute force'y, prob there are better solutions
  for end_pos in 0..max_val {
    let mut total_fuel_used = 0;
    for crab_pos in input {
      let fuel_used = (crab_pos - end_pos).abs();
      total_fuel_used += fuel_used;
    }
    if total_fuel_used < min_result {
      min_result = total_fuel_used;
    }
  }

  min_result
}

fn part_2(input: &Vec<i32>) -> f64 {
  let max_val = *input.iter().max().unwrap();
  let mut min_result = f64::INFINITY;

  for end_pos in 0..max_val {
    let mut total_fuel_used = 0.0;
    for crab_pos in input {
      let distance = (crab_pos - end_pos).abs() as f64;
      let fuel_used = (1.0 + distance) / 2.0 * distance;
      total_fuel_used += fuel_used;
    }
    if total_fuel_used < min_result {
      min_result = total_fuel_used;
    }
  }

  min_result
}

fn main() {
  let now = Instant::now();

  let input = read_input();
  println!("Part 1: {}", part_1(&input));
  println!("Part 2: {}", part_2(&input));

  let elapsed = now.elapsed();
  println!("Elapsed: {:.2?}", elapsed);
}
