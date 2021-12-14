use std::collections::HashMap;

type Rules = HashMap<String, [String; 2]>;

fn read_input() -> (String, Rules) {
  let lines: Vec<String> = aoc2021::get_input(14).collect();

  let template = lines[0].clone();
  let rules = lines
    .iter()
    .skip(2)
    .map(|rule| {
      let parts: Vec<&str> = rule.split(" -> ").collect();
      let pair = parts[0].to_string();
      let new_element = parts[1].chars().collect::<Vec<char>>()[0];

      let prev_elements: Vec<char> = pair.chars().collect();

      let new_pair_1 = format!("{}{}", prev_elements[0], new_element);
      let new_pair_2 = format!("{}{}", new_element, prev_elements[1]);

      (pair, [new_pair_1, new_pair_2])
    })
    .collect();

  (template, rules)
}

fn calc(iterations: usize, (template, rules): &(String, Rules)) -> u64 {
  let mut map: HashMap<&str, u64> = HashMap::new();
  for i in 0..template.len() - 1 {
    let new_pair = &template[i..i + 2];
    map
      .entry(new_pair)
      .and_modify(|old_val| *old_val += 1)
      .or_insert(1);
  }

  for _ in 0..iterations {
    let mut next_map: HashMap<&str, u64> = HashMap::new();

    for (pair, count) in map.iter() {
      let new_pairs = rules.get(*pair).unwrap();

      for new_pair in new_pairs {
        next_map
          .entry(new_pair)
          .and_modify(|old_val| *old_val += count)
          .or_insert(*count);
      }
    }

    map = next_map;
  }

  let mut counters: HashMap<char, u64> = HashMap::new();

  for (pair, count) in map {
    pair.chars().for_each(|char| {
      counters
        .entry(char)
        .and_modify(|old_val| *old_val += count)
        .or_insert(count);
    });
  }

  (counters.values().max().unwrap() - counters.values().min().unwrap()) / 2
}

fn part_1(input: &(String, Rules)) -> u64 {
  calc(10, input)
}

fn part_2(input: &(String, Rules)) -> u64 {
  calc(40, input)
}

fn main() {
  aoc2021::run(read_input, part_1, part_2)
}
