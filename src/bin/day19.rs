use std::{
  collections::{HashMap, HashSet},
  fmt::{self, Display},
  hash::Hash,
  ops::{Add, Neg},
};

use itertools::Itertools;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
struct Point {
  x: i32,
  y: i32,
  z: i32,
}

impl Point {
  fn new(x: i32, y: i32, z: i32) -> Point {
    Point { x, y, z }
  }

  fn distance(&self, point: &Point) -> f64 {
    let dist_part_1 =
      ((point.x - self.x).pow(2) + (point.y - self.y).pow(2) + (point.z - self.z).pow(2)) as f64;

    dist_part_1.sqrt()
  }

  fn manhattan_distance(&self, point: &Point) -> i32 {
    (self.x - point.x).abs() + (self.y - point.y).abs() + (self.z - point.z).abs()
  }
}

impl Add for Point {
  type Output = Self;

  fn add(self, other: Self) -> Self {
    Self {
      x: self.x + other.x,
      y: self.y + other.y,
      z: self.z + other.z,
    }
  }
}

impl Neg for Point {
  type Output = Self;
  fn neg(self) -> Self::Output {
    Self {
      x: -self.x,
      y: -self.y,
      z: -self.z,
    }
  }
}

impl Display for Point {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(f, "{},{},{}", self.x, self.y, self.z)
  }
}

// if there are gods, please forgive me for I have sinned
// I can't even imagine what are those 24 cases so I'm just gonna go with 48
fn get_subtractions_map() -> HashMap<&'static str, fn(a: Point, b: Point) -> Point> {
  let mut diffs_map: HashMap<&'static str, fn(a: Point, b: Point) -> Point> = HashMap::new();

  diffs_map.insert("- x, - y, - z", |a, b| {
    Point::new(a.x - b.x, a.y - b.y, a.z - b.z)
  });
  diffs_map.insert("+ x, - y, - z", |a, b| {
    Point::new(a.x + b.x, a.y - b.y, a.z - b.z)
  });
  diffs_map.insert("- x, + y, - z", |a, b| {
    Point::new(a.x - b.x, a.y + b.y, a.z - b.z)
  });
  diffs_map.insert("- x, - y, + z", |a, b| {
    Point::new(a.x - b.x, a.y - b.y, a.z + b.z)
  });
  diffs_map.insert("+ x, - y, + z", |a, b| {
    Point::new(a.x + b.x, a.y - b.y, a.z + b.z)
  });
  diffs_map.insert("+ x, + y, - z", |a, b| {
    Point::new(a.x + b.x, a.y + b.y, a.z - b.z)
  });
  diffs_map.insert("- x, + y, + z", |a, b| {
    Point::new(a.x - b.x, a.y + b.y, a.z + b.z)
  });
  diffs_map.insert("+ x, + y, + z", |a, b| {
    Point::new(a.x + b.x, a.y + b.y, a.z + b.z)
  });
  diffs_map.insert("- x, - z, - y", |a, b| {
    Point::new(a.x - b.x, a.y - b.z, a.z - b.y)
  });
  diffs_map.insert("+ x, - z, - y", |a, b| {
    Point::new(a.x + b.x, a.y - b.z, a.z - b.y)
  });
  diffs_map.insert("- x, - z, + y", |a, b| {
    Point::new(a.x - b.x, a.y - b.z, a.z + b.y)
  });
  diffs_map.insert("- x, + z, - y", |a, b| {
    Point::new(a.x - b.x, a.y + b.z, a.z - b.y)
  });
  diffs_map.insert("+ x, + z, - y", |a, b| {
    Point::new(a.x + b.x, a.y + b.z, a.z - b.y)
  });
  diffs_map.insert("+ x, - z, + y", |a, b| {
    Point::new(a.x + b.x, a.y - b.z, a.z + b.y)
  });
  diffs_map.insert("- x, + z, + y", |a, b| {
    Point::new(a.x - b.x, a.y + b.z, a.z + b.y)
  });
  diffs_map.insert("+ x, + z, + y", |a, b| {
    Point::new(a.x + b.x, a.y + b.z, a.z + b.y)
  });
  diffs_map.insert("- y, - x, - z", |a, b| {
    Point::new(a.x - b.y, a.y - b.x, a.z - b.z)
  });
  diffs_map.insert("+ y, - x, - z", |a, b| {
    Point::new(a.x + b.y, a.y - b.x, a.z - b.z)
  });
  diffs_map.insert("- y, + x, - z", |a, b| {
    Point::new(a.x - b.y, a.y + b.x, a.z - b.z)
  });
  diffs_map.insert("- y, - x, + z", |a, b| {
    Point::new(a.x - b.y, a.y - b.x, a.z + b.z)
  });
  diffs_map.insert("+ y, - x, + z", |a, b| {
    Point::new(a.x + b.y, a.y - b.x, a.z + b.z)
  });
  diffs_map.insert("+ y, + x, - z", |a, b| {
    Point::new(a.x + b.y, a.y + b.x, a.z - b.z)
  });
  diffs_map.insert("- y, + x, + z", |a, b| {
    Point::new(a.x - b.y, a.y + b.x, a.z + b.z)
  });
  diffs_map.insert("+ y, + x, + z", |a, b| {
    Point::new(a.x + b.y, a.y + b.x, a.z + b.z)
  });
  diffs_map.insert("- y, - z, - x", |a, b| {
    Point::new(a.x - b.y, a.y - b.z, a.z - b.x)
  });
  diffs_map.insert("+ y, - z, - x", |a, b| {
    Point::new(a.x + b.y, a.y - b.z, a.z - b.x)
  });
  diffs_map.insert("- y, + z, - x", |a, b| {
    Point::new(a.x - b.y, a.y + b.z, a.z - b.x)
  });
  diffs_map.insert("- y, - z, + x", |a, b| {
    Point::new(a.x - b.y, a.y - b.z, a.z + b.x)
  });
  diffs_map.insert("+ y, - z, + x", |a, b| {
    Point::new(a.x + b.y, a.y - b.z, a.z + b.x)
  });
  diffs_map.insert("+ y, + z, - x", |a, b| {
    Point::new(a.x + b.y, a.y + b.z, a.z - b.x)
  });
  diffs_map.insert("- y, + z, + x", |a, b| {
    Point::new(a.x - b.y, a.y + b.z, a.z + b.x)
  });
  diffs_map.insert("+ y, + z, + x", |a, b| {
    Point::new(a.x + b.y, a.y + b.z, a.z + b.x)
  });
  diffs_map.insert("- z, - y, - x", |a, b| {
    Point::new(a.x - b.z, a.y - b.y, a.z - b.x)
  });
  diffs_map.insert("+ z, - y, - x", |a, b| {
    Point::new(a.x + b.z, a.y - b.y, a.z - b.x)
  });
  diffs_map.insert("- z, + y, - x", |a, b| {
    Point::new(a.x - b.z, a.y + b.y, a.z - b.x)
  });
  diffs_map.insert("- z, - y, + x", |a, b| {
    Point::new(a.x - b.z, a.y - b.y, a.z + b.x)
  });
  diffs_map.insert("+ z, - y, + x", |a, b| {
    Point::new(a.x + b.z, a.y - b.y, a.z + b.x)
  });
  diffs_map.insert("+ z, + y, - x", |a, b| {
    Point::new(a.x + b.z, a.y + b.y, a.z - b.x)
  });
  diffs_map.insert("- z, + y, + x", |a, b| {
    Point::new(a.x - b.z, a.y + b.y, a.z + b.x)
  });
  diffs_map.insert("+ z, + y, + x", |a, b| {
    Point::new(a.x + b.z, a.y + b.y, a.z + b.x)
  });
  diffs_map.insert("- z, - x, - y", |a, b| {
    Point::new(a.x - b.z, a.y - b.x, a.z - b.y)
  });
  diffs_map.insert("+ z, - x, - y", |a, b| {
    Point::new(a.x + b.z, a.y - b.x, a.z - b.y)
  });
  diffs_map.insert("- z, + x, - y", |a, b| {
    Point::new(a.x - b.z, a.y + b.x, a.z - b.y)
  });
  diffs_map.insert("- z, - x, + y", |a, b| {
    Point::new(a.x - b.z, a.y - b.x, a.z + b.y)
  });
  diffs_map.insert("+ z, - x, + y", |a, b| {
    Point::new(a.x + b.z, a.y - b.x, a.z + b.y)
  });
  diffs_map.insert("+ z, + x, - y", |a, b| {
    Point::new(a.x + b.z, a.y + b.x, a.z - b.y)
  });
  diffs_map.insert("- z, + x, + y", |a, b| {
    Point::new(a.x - b.z, a.y + b.x, a.z + b.y)
  });
  diffs_map.insert("+ z, + x, + y", |a, b| {
    Point::new(a.x + b.z, a.y + b.x, a.z + b.y)
  });

  diffs_map
}

fn read_input() -> Vec<Vec<Point>> {
  let mut scans = vec![];
  let mut scan_results = vec![];

  for line in aoc2021::get_input(19) {
    if line.is_empty() {
      scans.push(scan_results.clone());
    } else if line.contains("scanner") {
      scan_results = vec![];
    } else {
      let (x, y, z) = line
        .split(',')
        .flat_map(|val| val.parse::<i32>())
        .collect_tuple()
        .unwrap();
      let point = Point { x, y, z };
      scan_results.push(point);
    }
  }

  if !scan_results.is_empty() {
    scans.push(scan_results);
  }

  scans
}

fn find_beacons_and_scanners(
  input: &[Vec<Point>],
) -> (Vec<HashMap<Point, HashSet<String>>>, Vec<Point>) {
  let subtractions_map = get_subtractions_map();
  let mut scans = vec![];

  for scan in input {
    let mut map = HashMap::new();
    for ref_point in scan.iter() {
      let mut set: HashSet<String> = HashSet::new();
      for point in scan.iter() {
        let dist = ref_point.distance(point);
        set.insert(format!("{:.10}", dist));
      }
      map.insert(*ref_point, set);
    }
    scans.push(map);
  }

  let mut scan_to_move = None;
  let mut translated_scans = vec![scans.swap_remove(0)];
  let mut scanners = vec![];

  loop {
    let mut normalized_map: HashMap<Point, HashSet<String>> = HashMap::new();

    'main: for (idx, scan) in scans.iter().enumerate() {
      let mut count = 0;
      let mut subtraction_results: HashMap<&str, Option<Point>> = HashMap::new();

      for scan_to_check in translated_scans.iter().rev() {
        for (point_0, distances_0) in scan_to_check {
          for (point_1, distances_1) in scan.iter() {
            if distances_0.intersection(distances_1).count() >= 12 {
              count += 1;

              for (action, fun) in subtractions_map.iter() {
                if !subtraction_results.contains_key(action) {
                  subtraction_results.insert(action, Some(fun(*point_0, *point_1)));
                } else {
                  let result = fun(*point_0, *point_1);
                  if let Some(old_result) = subtraction_results[action] {
                    if result != old_result {
                      subtraction_results.insert(action, None);
                    }
                  }
                }
              }
            }
          }

          if count >= 12 {
            let (action, scanner_pos) = subtraction_results
              .iter()
              .filter(|(_, scan_pos)| !scan_pos.is_none())
              .map(|(action, scan_pos)| (action, scan_pos.unwrap()))
              .last()
              .unwrap();

            let fun = subtractions_map[action];
            scanners.push(scanner_pos);

            for (point, distances) in scan {
              normalized_map.insert(fun(scanner_pos, -*point), distances.clone());
            }
            scan_to_move = Some(idx);

            break 'main;
          }
        }
      }
    }

    translated_scans.push(normalized_map);

    if let Some(index) = scan_to_move {
      scans.swap_remove(index);
      scan_to_move = None;
    } else {
      break;
    }
  }

  (translated_scans, scanners)
}

#[allow(clippy::ptr_arg)]
fn part_1(input: &Vec<Vec<Point>>) -> usize {
  find_beacons_and_scanners(input)
    .0
    .iter()
    .flat_map(|map| map.keys())
    .collect::<HashSet<&Point>>()
    .len()
}

#[allow(clippy::ptr_arg)]
fn part_2(input: &Vec<Vec<Point>>) -> i32 {
  let scanners = find_beacons_and_scanners(input).1;

  scanners
    .iter()
    .cartesian_product(&scanners)
    .filter(|(a, b)| *a != *b)
    .map(|(a, b)| a.manhattan_distance(b))
    .max()
    .unwrap()
}

fn main() {
  aoc2021::run(read_input, part_1, part_2)
}
