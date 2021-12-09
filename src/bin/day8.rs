use aoc2021::get_input;
use itertools::Itertools;
use std::time::Instant;
use std::{collections::HashMap, io::BufRead};

fn read_input() -> Vec<(Vec<String>, Vec<String>)> {
  get_input(8)
    .lines()
    .map(|line| {
      let (patterns, digits): (Vec<String>, Vec<String>) = line
        .expect("Couldn't read line")
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

fn common_segments(a: &str, b: &Vec<char>) -> Vec<char> {
  a.chars()
    .filter(|item| b.contains(item))
    .collect::<Vec<char>>()
}

fn get_number(patterns: &Vec<String>, numbers: &Vec<String>) -> u32 {
  let mut segments = ['-'; 7];
  let mut digits = [""; 10];

  // warning: write only code, things like this are SO HARD IN RUST HOLY SHIT
  while segments.iter().any(|segment| *segment == '-') {
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
        5 => {
          // digit 3: segments 0 1 2 3 6
          // when compared with digits 7 and then 4, we can figure out segments 5 and 6
          if digits[7]
            .chars()
            .all(|char| pattern.chars().contains(&char))
          {
            digits[3] = pattern;

            segments[6] = common_segments(digits[4], &digit_diff(digits[3], digits[7]))[0];

            let mut seg_126 = String::from(digits[1]);
            seg_126.push(segments[6]);

            segments[5] = digit_diff(digits[4], seg_126.as_str())[0];
          // digit 5: segment 0 2 3 5 6
          // the only 5-long digit that contains segment 5 which we figured out earlier in digit 3
          // based on this one we can figure out the rest of segments
          } else if segments[5] != '-' && pattern.contains(segments[5]) {
            digits[5] = pattern;

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
          } else if segments[1] != '-' {
            digits[2] = pattern;

            let seg_0136 = format!(
              "{}{}{}{}",
              segments[0], segments[1], segments[3], segments[6]
            );
            segments[4] = digit_diff(digits[2], &seg_0136)[0];
          }
        }
        _ => (),
      }
    }
  }

  let len_6_iter: Vec<&String> = patterns.iter().filter(|pat| pat.len() == 6).collect();

  digits[0] = len_6_iter
    .iter()
    .find(|pat| !pat.contains(segments[6]))
    .unwrap();
  digits[6] = len_6_iter
    .iter()
    .find(|pat| !pat.contains(segments[1]))
    .unwrap();
  digits[9] = len_6_iter
    .iter()
    .find(|pat| !pat.contains(segments[4]))
    .unwrap();

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

fn part_2(input: &Vec<(Vec<String>, Vec<String>)>) -> u32 {
  input
    .iter()
    .map(|(patterns, digits)| get_number(patterns, digits))
    .sum()
}

fn main() {
  let now = Instant::now();

  let input = read_input();
  println!("Part 1: {}", part_1(&input));
  println!("Part 2: {}", part_2(&input));

  let elapsed = now.elapsed();
  println!("Elapsed: {:.2?}", elapsed);
}
