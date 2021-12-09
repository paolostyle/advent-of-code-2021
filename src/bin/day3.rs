use aoc2021::get_input;
use std::{cmp::Ordering, io::BufRead, time::Instant};

type Column = Vec<char>;
type Matrix = Vec<Column>;

fn read_input() -> Matrix {
  get_input(3)
    .lines()
    .map(|line| line.expect("could not read line").chars().collect())
    .collect()
}

#[derive(Copy, Clone, Debug)]
struct BitCounter {
  zeroes: u16,
  ones: u16,
}

impl BitCounter {
  fn new() -> BitCounter {
    BitCounter { zeroes: 0, ones: 0 }
  }

  fn count(&mut self, column: &Column) {
    for bit in column {
      match bit {
        '0' => self.zeroes += 1,
        '1' => self.ones += 1,
        _ => panic!("Non binary value!"),
      }
    }
  }

  fn most_common_bit(&self, prefered: char) -> char {
    match self.ones.cmp(&self.zeroes) {
      Ordering::Greater => '1',
      Ordering::Less => '0',
      Ordering::Equal => {
        if prefered != '1' && prefered != '0' {
          panic!("Non binary value!");
        } else {
          prefered
        }
      }
    }
  }

  fn least_common_bit(&self, prefered: char) -> char {
    match self.ones.cmp(&self.zeroes) {
      Ordering::Greater => '0',
      Ordering::Less => '1',
      Ordering::Equal => {
        if prefered != '1' && prefered != '0' {
          panic!("This should never happen!");
        } else {
          prefered
        }
      }
    }
  }
}

fn get_column(input: &Matrix, column_number: usize) -> Column {
  input.iter().map(|num| num[column_number]).collect()
}

fn matrix_to_binary(matrix: &Column) -> String {
  matrix.iter().collect::<String>()
}

fn binary_to_decimal(binary: String) -> u32 {
  u32::from_str_radix(binary.as_str(), 2).unwrap()
}

fn part_1(input: &Matrix) -> u32 {
  let bits = input[0].len();
  let mut counters: Vec<BitCounter> = vec![BitCounter::new(); bits];

  for column_number in 0..bits {
    let column = get_column(input, column_number);
    counters[column_number].count(&column);
  }

  let mut gamma = String::new();
  let mut epsilon = String::new();

  for counter in counters {
    gamma.push(counter.most_common_bit('?'));
    epsilon.push(counter.least_common_bit('?'));
  }

  binary_to_decimal(gamma) * binary_to_decimal(epsilon)
}

fn part_2(input: &Matrix) -> u32 {
  let mut oxygen_candidates = input.clone();
  let mut scrubber_candidates = input.clone();
  let mut active_bit = 0;

  while oxygen_candidates.len() != 1 {
    let mut counter = BitCounter::new();
    let column = get_column(&oxygen_candidates, active_bit);

    counter.count(&column);

    let most_common_bit = counter.most_common_bit('1');
    oxygen_candidates.retain(|val| val[active_bit] == most_common_bit);
    active_bit += 1;
  }

  active_bit = 0;

  while scrubber_candidates.len() != 1 {
    let mut counter = BitCounter::new();
    let column = get_column(&scrubber_candidates, active_bit);

    counter.count(&column);

    let least_common_bit = counter.least_common_bit('0');
    scrubber_candidates.retain(|val| val[active_bit] == least_common_bit);
    active_bit += 1;
  }

  let oxygen_value = binary_to_decimal(matrix_to_binary(&oxygen_candidates[0]));
  let scrubber_value = binary_to_decimal(matrix_to_binary(&scrubber_candidates[0]));

  oxygen_value * scrubber_value
}

fn main() {
  let now = Instant::now();

  let input = read_input();
  println!("Part 1: {}", part_1(&input));
  println!("Part 2: {}", part_2(&input));

  let elapsed = now.elapsed();
  println!("Elapsed: {:.2?}", elapsed);
}
