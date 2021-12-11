use std::{fmt::Display, fs::File, io::BufRead, io::BufReader, time::Instant};

use num::Integer;

fn load_file(path: String) -> impl Iterator<Item = String> {
  let input_file = File::open(path).expect("Input file doesn't exist");
  BufReader::new(input_file).lines().flatten()
}

pub fn get_input(day: u8) -> impl Iterator<Item = String> {
  load_file(format!("inputs/day{}.txt", day))
}

pub fn get_test_input(day: u8) -> impl Iterator<Item = String> {
  load_file(format!("test_inputs/day{}.txt", day))
}

pub fn run<T, R1: Integer + Display, R2: Integer + Display>(
  read_input: fn() -> T,
  part_1: fn(&T) -> R1,
  part_2: fn(&T) -> R2,
) {
  let now = Instant::now();

  let input = read_input();
  println!("Part 1: {}", part_1(&input));
  println!("Part 2: {}", part_2(&input));

  let time = now.elapsed();
  println!("Execution time: {:.2?}", time);
}
