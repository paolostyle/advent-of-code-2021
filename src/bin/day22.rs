use std::collections::HashSet;

use regex::Regex;

#[derive(Clone, Copy, Debug)]
struct Setting {
  on: bool,
  x: [i64; 2],
  y: [i64; 2],
  z: [i64; 2],
}

impl Setting {
  fn new(on: bool, settings: Vec<i64>) -> Setting {
    Setting {
      on,
      x: [settings[0], settings[1]],
      y: [settings[2], settings[3]],
      z: [settings[4], settings[5]],
    }
  }

  fn is_in_initialization(&self) -> bool {
    let range = -50..=50;
    [self.x, self.y, self.z]
      .iter()
      .flatten()
      .all(|val| range.contains(val))
  }

  fn get_list(&self) -> Vec<(i64, i64, i64)> {
    let mut vec = vec![];

    for x in self.x[0]..=self.x[1] {
      for y in self.y[0]..=self.y[1] {
        for z in self.z[0]..=self.z[1] {
          vec.push((x, y, z));
        }
      }
    }

    vec
  }
}

fn read_input() -> Vec<Setting> {
  let re =
    Regex::new(r"(on|off) x=(-?\d+)\.\.(-?\d+),y=(-?\d+)\.\.(-?\d+),z=(-?\d+)\.\.(-?\d+)").unwrap();

  aoc2021::get_test_input(22)
    .map(|line| {
      let mut on = true;
      let mut settings = Vec::with_capacity(6);

      let captures = re.captures(&line).unwrap();
      for i in 1..8 {
        if i == 1 {
          on = captures[i] == *"on";
        } else {
          settings.push(captures[i].parse::<i64>().unwrap())
        }
      }

      Setting::new(on, settings)
    })
    .collect()
}

#[allow(clippy::ptr_arg)]
fn part_1(input: &Vec<Setting>) -> usize {
  let init_settings: Vec<&Setting> = input
    .iter()
    .filter(|setting| setting.is_in_initialization())
    .collect();

  let mut cubes: HashSet<(i64, i64, i64)> = HashSet::new();

  for setting in init_settings {
    for el in setting.get_list() {
      if setting.on {
        cubes.insert(el);
      } else {
        cubes.remove(&el);
      }
    }
  }

  cubes.len()
}

#[allow(clippy::ptr_arg)]
fn part_2(input: &Vec<Setting>) -> usize {
  0
}

fn main() {
  aoc2021::run(read_input, part_1, part_2)
}
