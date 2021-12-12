use itertools::Itertools;
use std::collections::HashMap;

fn read_input() -> Vec<(Vec<String>, Vec<String>)> {
  aoc2021::get_input(8)
    .map(|line| {
      let (patterns, digits): (Vec<String>, Vec<String>) = line
        .split(" | ")
        .into_iter()
        .map(|part| {
          part
            .split_whitespace()
            .map(|val| val.chars().sorted().join(""))
            .collect()
        })
        .next_tuple()
        .unwrap();

      (patterns, digits)
    })
    .collect()
}

fn digit_diff(a: &str, b: &str) -> Vec<char> {
  let b_chars_vec: Vec<char> = b.chars().collect();
  a.chars()
    .filter(|item| !b_chars_vec.contains(item))
    .collect::<Vec<char>>()
}

fn common_segments(a: &str, b: &[char]) -> Vec<char> {
  a.chars()
    .filter(|item| b.contains(item))
    .collect::<Vec<char>>()
}

fn get_number(patterns: &[String], numbers: &[String]) -> u32 {
  let mut segments = ['-'; 7];
  let mut digits = [""; 10];

  let mut len_5_patterns: Vec<&String> = Vec::with_capacity(3);
  let mut len_6_patterns: Vec<&String> = Vec::with_capacity(3);

  for pattern in patterns.iter().sorted_by(|a, b| a.len().cmp(&b.len())) {
    match pattern.len() {
      // digit 1: segments 1 2
      2 => digits[1] = pattern,
      // digit 7: segments 0 1 2
      // segment 0 can be figured out by comparing with digit 1
      3 => {
        digits[7] = pattern;
        segments[0] = digit_diff(digits[7], digits[1])[0];
      }
      // digit 4: segments 1 2 5 6
      4 => digits[4] = pattern,
      // digit 8: segments 0 1 2 3 4 5 6
      7 => digits[8] = pattern,
      5 => len_5_patterns.push(pattern),
      6 => len_6_patterns.push(pattern),
      _ => (),
    }
  }

  // warning: write only code, things like this are SO HARD IN RUST HOLY SHIT
  // digit 3: segments 0 1 2 3 6
  // when compared with digits 7 and then 4, we can figure out segments 5 and 6
  digits[3] = *len_5_patterns
    .iter()
    .find(|pattern| {
      digits[7]
        .chars()
        .all(|char| pattern.chars().contains(&char))
    })
    .unwrap();

  segments[6] = common_segments(digits[4], &digit_diff(digits[3], digits[7]))[0];

  let mut seg_126 = String::from(digits[1]);
  seg_126.push(segments[6]);

  segments[5] = digit_diff(digits[4], seg_126.as_str())[0];

  // digit 5: segment 0 2 3 5 6
  // the only 5-long digit that contains segment 5 which we figured out earlier in digit 3
  // based on this one we can figure out the rest of segments
  digits[5] = len_5_patterns
    .iter()
    .find(|pattern| pattern.contains(segments[5]))
    .unwrap();

  let seg_056 = format!("{}{}{}", segments[0], segments[5], segments[6]);
  let seg_23 = digit_diff(digits[5], &seg_056);
  for digit in seg_23 {
    if digits[1].contains(digit) {
      segments[2] = digit;
    } else {
      segments[3] = digit;
    }
  }
  segments[1] = digit_diff(digits[1], &String::from(segments[2]))[0];

  // digit 2: segments 0 1 3 4 6
  // the last 5 digit, requires previous 2 to be solved, after that we have all the segments
  digits[2] = len_5_patterns
    .iter()
    .find(|pattern| pattern.contains(segments[1]) && !pattern.contains(segments[2]))
    .unwrap();

  let seg_0136 = format!(
    "{}{}{}{}",
    segments[0], segments[1], segments[3], segments[6]
  );
  segments[4] = digit_diff(digits[2], &seg_0136)[0];

  let digit_to_missing_segment_map = [(0, 6), (6, 1), (9, 4)];

  for (digit, segment) in digit_to_missing_segment_map {
    digits[digit] = len_6_patterns
      .iter()
      .find(|pat| !pat.contains(segments[segment]))
      .unwrap();
  }

  let mut digits_map: HashMap<&str, String> = HashMap::new();
  for (digit, code) in digits.iter().enumerate() {
    digits_map.insert(*code, digit.to_string());
  }

  numbers
    .iter()
    .map(|code| digits_map.get(code.as_str()).unwrap())
    .join("")
    .parse()
    .expect("Couldn't parse number")
}

#[allow(clippy::ptr_arg)]
fn part_1(input: &Vec<(Vec<String>, Vec<String>)>) -> usize {
  input
    .iter()
    .map(|(_, digits)| {
      digits
        .iter()
        .map(|digit| digit.len())
        .filter(|len| [2, 3, 4, 7].iter().any(|val| val == len))
        .count()
    })
    .sum()
}

#[allow(clippy::ptr_arg)]
fn part_2(input: &Vec<(Vec<String>, Vec<String>)>) -> u32 {
  input
    .iter()
    .map(|(patterns, digits)| get_number(patterns, digits))
    .sum()
}

fn main() {
  aoc2021::run(read_input, part_1, part_2)
}
