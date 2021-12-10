use aoc2021::get_input;
use std::{collections::HashMap, io::BufRead, time::Instant};

const OPENING_BRACES: [char; 4] = ['(', '[', '{', '<'];
const BRACES_PAIRS: [(char, char); 4] = [(')', '('), (']', '['), ('}', '{'), ('>', '<')];
const REV_BRACES_PAIRS: [(char, char); 4] = [('(', ')'), ('[', ']'), ('{', '}'), ('<', '>')];

fn read_input() -> Vec<String> {
  get_input(10).lines().flatten().collect()
}

fn part_1(input: &Vec<String>) -> u32 {
  let braces_map = HashMap::from(BRACES_PAIRS);
  let mut score = 0;

  for line in input {
    let mut stack = Vec::with_capacity(input.len());
    for brace in line.chars() {
      if OPENING_BRACES.contains(&brace) {
        stack.push(brace);
      } else {
        if braces_map.get(&brace) == stack.last() {
          stack.pop();
        } else {
          score += match brace {
            ')' => 3,
            ']' => 57,
            '}' => 1197,
            '>' => 25137,
            _ => panic!("!"),
          };
          break;
        }
      }
    }
  }

  score
}

fn part_2(input: &Vec<String>) -> u64 {
  let braces_map = HashMap::from(BRACES_PAIRS);
  let rev_braces_map = HashMap::from(REV_BRACES_PAIRS);
  let mut scores = Vec::new();

  for line in input {
    let mut stack: Vec<char> = Vec::with_capacity(input.len());
    let mut is_corrupted = false;

    for brace in line.chars() {
      if OPENING_BRACES.contains(&brace) {
        stack.push(brace);
      } else {
        if braces_map.get(&brace) == stack.last() {
          stack.pop();
        } else {
          is_corrupted = true;
          break;
        }
      }
    }

    if !is_corrupted {
      let mut score: u64 = 0;
      for brace in stack.iter().rev() {
        let enclosing_brace = rev_braces_map[&brace];
        let points = match enclosing_brace {
          ')' => 1,
          ']' => 2,
          '}' => 3,
          '>' => 4,
          _ => panic!("!"),
        };

        score = score * 5 + points;
      }
      scores.push(score);
    }
  }

  scores.sort();
  let mid = scores.len() / 2;
  scores[mid]
}

fn main() {
  let now = Instant::now();

  let input = read_input();
  println!("Part 1: {}", part_1(&input));
  println!("Part 2: {}", part_2(&input));

  let elapsed = now.elapsed();
  println!("Elapsed: {:.2?}", elapsed);
}
