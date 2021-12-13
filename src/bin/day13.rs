use std::{collections::HashSet, fmt};

use itertools::Itertools;

type Dot = (i32, i32);
type Fold = (char, i32);

struct Paper {
  map: Vec<Vec<char>>,
}

impl Paper {
  fn new(set: HashSet<Dot>) -> Paper {
    let size_x = set.iter().max_by(|(x1, _), (x2, _)| x1.cmp(x2)).unwrap().0 as usize + 1;
    let size_y = set.iter().max_by(|(_, y1), (_, y2)| y1.cmp(y2)).unwrap().1 as usize + 1;

    let mut map = vec![vec![' '; size_x]; size_y];

    for (x, y) in set {
      map[y as usize][x as usize] = '•';
    }

    Paper { map }
  }
}

impl fmt::Display for Paper {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    let mut final_output = String::from('\n');
    for row in self.map.iter() {
      let str: String = row.iter().collect();
      final_output.push_str(&str);
      final_output.push('\n');
    }
    writeln!(f, "{}", final_output)
  }
}

fn read_input() -> (Vec<Dot>, Vec<Fold>) {
  let mut separator_encountered = false;
  let mut dots: Vec<Dot> = vec![];
  let mut folds: Vec<Fold> = vec![];

  for line in aoc2021::get_input(13) {
    if line.is_empty() {
      separator_encountered = true;
      continue;
    }

    if !separator_encountered {
      let pair: (i32, i32) = line
        .split(',')
        .flat_map(|coord| coord.parse::<i32>())
        .collect_tuple()
        .unwrap();

      dots.push(pair);
    } else {
      let pair: (&str, &str) = line[11..].split('=').collect_tuple().unwrap();
      let dimension = pair.0.chars().next().unwrap();
      let value = pair.1.parse::<i32>().unwrap();

      folds.push((dimension, value));
    }
  }

  (dots, folds)
}

fn fold_paper((fold_by, fold): &Fold, dots_set: &HashSet<Dot>) -> HashSet<Dot> {
  let mut set: HashSet<Dot> = HashSet::new();

  for dot in dots_set {
    let pair = match fold_by {
      'x' => {
        let x = if dot.0 > *fold {
          fold - (dot.0 - fold)
        } else {
          dot.0
        };
        (x, dot.1)
      }
      'y' => {
        let y = if dot.1 > *fold {
          fold - (dot.1 - fold)
        } else {
          dot.1
        };
        (dot.0, y)
      }
      _ => panic!("?"),
    };

    set.insert(pair);
  }

  set
}

#[allow(clippy::ptr_arg)]
fn part_1((dots, folds): &(Vec<Dot>, Vec<Fold>)) -> usize {
  let set: HashSet<Dot> = HashSet::from_iter(dots.iter().cloned());

  let new_set = fold_paper(&folds[0], &set);

  new_set.len()
}

#[allow(clippy::ptr_arg)]
fn part_2((dots, folds): &(Vec<Dot>, Vec<Fold>)) -> Paper {
  let mut set: HashSet<Dot> = HashSet::from_iter(dots.iter().cloned());

  for fold in folds {
    set = fold_paper(fold, &set);
  }

  Paper::new(set)
}

fn main() {
  aoc2021::run(read_input, part_1, part_2)
}
