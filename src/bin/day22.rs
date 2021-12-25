#![allow(dead_code)]
use std::{cmp::Ordering, collections::HashSet, ops::Index};

use regex::Regex;

enum Dimension {
  X,
  Y,
  Z,
}

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

impl Index<&Dimension> for Setting {
  type Output = [i64; 2];

  fn index(&self, dim: &Dimension) -> &Self::Output {
    match dim {
      Dimension::X => &self.x,
      Dimension::Y => &self.y,
      Dimension::Z => &self.z,
    }
  }
}

fn read_input() -> Vec<Setting> {
  let re =
    Regex::new(r"(on|off) x=(-?\d+)\.\.(-?\d+),y=(-?\d+)\.\.(-?\d+),z=(-?\d+)\.\.(-?\d+)").unwrap();

  aoc2021::get_input(22)
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

fn get_lists(input: &[Setting], dim: &Dimension) -> Vec<[i64; 2]> {
  let mut lists = vec![[0, 0]];

  for setting in input {
    let [lower, upper] = setting[dim];
    let mut maybe_new = false;
    let mut def_new = vec![];

    for list in lists.iter_mut() {
      let is_disjoint =
        (lower < list[0] && upper < list[0]) || (lower > list[1] && upper > list[1]);

      if setting.on {
        if is_disjoint {
          maybe_new = true;
        } else {
          if lower < list[0] {
            list[0] = lower;
          }
          if upper > list[1] {
            list[1] = upper;
          }

          maybe_new = false;
        }
      } else if !is_disjoint {
        if lower > list[0] && upper < list[1] {
          def_new.push([upper, list[1]]);
          list[1] = lower;
        } else if lower <= list[0] && upper < list[1] {
          list[0] = upper;
        } else if lower <= list[0] && upper >= list[1] {
          list[0] = 0;
          list[1] = 0;
        } else if lower > list[0] && upper >= list[1] {
          list[1] = lower;
        }
      }
    }

    if maybe_new {
      lists.push([lower, upper]);
    }

    lists.append(&mut def_new);
    lists.retain(|list| list[0] != 0 && list[1] != 0);

    let mut to_remove = vec![];
    for i in 0..lists.len() {
      for j in i + 1..lists.len() {
        if i != j {
          if lists[i][0] == lists[j][0] {
            match lists[i][1].cmp(&lists[j][1]) {
              Ordering::Equal => to_remove.push(i),
              Ordering::Greater => to_remove.push(j),
              Ordering::Less => to_remove.push(i),
            }
          }

          if lists[i][1] == lists[j][1] {
            match lists[i][0].cmp(&lists[j][0]) {
              Ordering::Equal => to_remove.push(i),
              Ordering::Greater => to_remove.push(i),
              Ordering::Less => to_remove.push(j),
            }
          }
        }
      }
    }

    if !to_remove.is_empty() {
      lists = lists
        .into_iter()
        .enumerate()
        .filter(|(idx, _)| !to_remove.contains(idx))
        .map(|(_, val)| val)
        .collect();
    }
  }

  lists
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
  println!("{:?}", get_lists(&input[0..20], &Dimension::Z));

  0
}

fn main() {
  aoc2021::run(read_input, part_1, part_2)
}
