use std::fmt;

const MAP_SIZE: usize = 10;

struct OctopusGrid {
  grid: [[u32; MAP_SIZE]; MAP_SIZE],
  flashes: u32,
  flashes_synced: bool,
}

impl OctopusGrid {
  fn new(input: &[Vec<u32>]) -> OctopusGrid {
    let mut grid = [[0; MAP_SIZE]; MAP_SIZE];

    for (row_idx, row) in input.iter().enumerate() {
      for (idx, item) in row.iter().enumerate() {
        grid[row_idx][idx] = *item;
      }
    }

    OctopusGrid {
      grid,
      flashes: 0,
      flashes_synced: false,
    }
  }

  fn step(&mut self) {
    for row in self.grid.iter_mut() {
      for item in row.iter_mut() {
        *item += 1;
      }
    }

    let mut flashed = [[false; MAP_SIZE]; MAP_SIZE];
    let mut needs_another = false;

    loop {
      let mut changes = [[0; MAP_SIZE + 2]; MAP_SIZE + 2];

      for (row_idx, row) in self.grid.iter().enumerate() {
        for (idx, item) in row.iter().enumerate() {
          if *item > 9 && !flashed[row_idx][idx] {
            flashed[row_idx][idx] = true;
            for i in 0..3 {
              for j in 0..3 {
                if !(i == 1 && j == 1) {
                  changes[row_idx + i][idx + j] += 1;
                }
              }
            }
          }
        }
      }

      for (row_idx, row) in self.grid.iter_mut().enumerate() {
        for (idx, item) in row.iter_mut().enumerate() {
          *item += changes[row_idx + 1][idx + 1];
          if *item > 9 && !flashed[row_idx][idx] {
            needs_another = true;
          }
        }
      }

      if !needs_another {
        break;
      }

      needs_another = false;
    }

    for row in self.grid.iter_mut() {
      for item in row.iter_mut() {
        if *item > 9 {
          *item = 0;
        }
      }
    }

    let step_flashes = flashed.iter().flatten().filter(|flashed| **flashed).count() as u32;

    self.flashes += step_flashes;
    self.flashes_synced = step_flashes == (MAP_SIZE * MAP_SIZE) as u32;
  }
}

impl fmt::Display for OctopusGrid {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    let mut final_output = String::new();
    for row in self.grid.iter() {
      let str: String = row.iter().map(|digit| digit.to_string()).collect();
      final_output.push_str(&str);
      final_output.push('\n');
    }
    writeln!(f, "{}", final_output)
  }
}

fn read_input() -> Vec<Vec<u32>> {
  aoc2021::get_input(11)
    .map(|row| row.chars().flat_map(|item| item.to_digit(10)).collect())
    .collect()
}

#[allow(clippy::ptr_arg)]
fn part_1(input: &Vec<Vec<u32>>) -> u32 {
  let mut grid = OctopusGrid::new(input);

  for _ in 0..100 {
    grid.step();
  }

  grid.flashes
}

#[allow(clippy::ptr_arg)]
fn part_2(input: &Vec<Vec<u32>>) -> u32 {
  let mut grid = OctopusGrid::new(input);

  let mut step = 0;
  loop {
    grid.step();
    step += 1;

    if grid.flashes_synced {
      break step;
    }
  }
}

fn main() {
  aoc2021::run(read_input, part_1, part_2)
}
