use itertools::Itertools;
use lazy_static::lazy_static;
use regex::{Captures, Regex};

fn read_input() -> Vec<String> {
  aoc2021::get_input(18).collect()
}

fn string_sum(num1: &str, num2: &str) -> String {
  format!("[{},{}]", num1, num2)
}

fn explode(number: &str, repl_start: usize, repl_end: usize) -> String {
  let (left_number, right_number): (i32, i32) = number[repl_start + 1..repl_end]
    .to_string()
    .split(',')
    .flat_map(|number| number.parse())
    .collect_tuple()
    .unwrap();

  let mut before_pair = number[..repl_start].to_string();

  let mut digit_idx = None;
  for (idx, char) in before_pair.chars().rev().enumerate() {
    if char.is_digit(10) {
      digit_idx = Some(idx);
      break;
    }
  }

  if let Some(digit_idx) = digit_idx {
    let actual_idx = before_pair.len() - 1 - digit_idx;
    let prev_char = before_pair.as_bytes()[actual_idx - 1] as char;

    if prev_char.is_digit(10) {
      let (left, right) = before_pair.split_at(actual_idx - 1);
      let new_value = right[..2].parse::<i32>().unwrap() + left_number;
      before_pair = format!("{}{}{}", left, new_value, &right[2..]);
    } else {
      let (left, right) = before_pair.split_at(actual_idx);
      let new_value = right[..1].parse::<i32>().unwrap() + left_number;
      before_pair = format!("{}{}{}", left, new_value, &right[1..]);
    }
  }

  let mut after_pair = number[repl_end + 1..].to_string();

  digit_idx = None;
  for (idx, char) in after_pair.chars().enumerate() {
    if char.is_digit(10) {
      digit_idx = Some(idx);
      break;
    }
  }

  if let Some(digit_idx) = digit_idx {
    let next_char = after_pair.as_bytes()[digit_idx + 1] as char;
    let new_value_str = if next_char.is_digit(10) {
      &after_pair[digit_idx..digit_idx + 2]
    } else {
      &after_pair[digit_idx..digit_idx + 1]
    };
    let new_value = new_value_str.parse::<i32>().unwrap() + right_number;
    after_pair = after_pair.replacen(new_value_str, &new_value.to_string(), 1);
  }

  format!("{}0{}", before_pair, after_pair)
}

fn split(number: &str, idx: usize) -> String {
  let num_to_explode: f32 = number[idx..idx + 2].parse().unwrap();
  let left_number = (num_to_explode / 2.0).floor() as i32;
  let right_number = (num_to_explode / 2.0).ceil() as i32;
  let replaced_pair = format!("[{},{}]", left_number, right_number);

  number.replacen(&number[idx..idx + 2], &replaced_pair, 1)
}

fn reduce(number: &str) -> String {
  let chars: Vec<char> = number.chars().collect();

  let mut depth = 0;
  let mut stack: Vec<char> = Vec::with_capacity(10);

  let mut repl_start = 0;

  for (idx, char) in chars.iter().enumerate() {
    if *char == '[' {
      stack.push(*char);
      depth += 1;

      if depth > 4 {
        repl_start = idx;
      }
    }
    if *char == ']' {
      if depth > 4 && repl_start != 0 {
        return explode(number, repl_start, idx);
      }

      stack.pop();
      depth -= 1;
    }
  }

  for (idx, char) in chars.iter().enumerate() {
    if char.is_digit(10) && chars[idx + 1].is_digit(10) && repl_start == 0 {
      return split(number, idx);
    }
  }

  number.to_string()
}

fn add(num_1: &str, num_2: &str) -> String {
  let sum_str = string_sum(num_1, num_2);
  let mut reduced_str = reduce(&sum_str);

  loop {
    let further_reduced = reduce(&reduced_str);

    if further_reduced == reduced_str {
      break further_reduced;
    } else {
      reduced_str = further_reduced;
    }
  }
}

fn get_magnitude(number: &str) -> u32 {
  lazy_static! {
    static ref RE: Regex = Regex::new(r"\[(\d+),(\d+)]").unwrap();
  }

  let mut result = number.to_string();

  loop {
    let new_result = RE
      .replace_all(&result, |caps: &Captures| {
        let num_1: i32 = caps[1].parse().unwrap();
        let num_2: i32 = caps[2].parse().unwrap();
        format!("{}", 3 * num_1 + 2 * num_2)
      })
      .to_string();

    if new_result == result {
      break result = new_result;
    } else {
      result = new_result;
    }
  }

  result.parse().unwrap()
}

#[allow(clippy::ptr_arg)]
fn part_1(input: &Vec<String>) -> u32 {
  get_magnitude(
    &input
      .iter()
      .map(|s| s.to_string())
      .reduce(|prev, curr| add(&prev, &curr))
      .unwrap(),
  )
}

#[allow(clippy::ptr_arg)]
fn part_2(input: &Vec<String>) -> u32 {
  input
    .iter()
    .cartesian_product(input)
    .filter(|(a, b)| *a != *b)
    .map(|(a, b)| get_magnitude(&add(a, b)))
    .max()
    .unwrap()
}

fn main() {
  aoc2021::run(read_input, part_1, part_2)
}
