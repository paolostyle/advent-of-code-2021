fn read_input() -> Vec<i32> {
  aoc2021::get_input(7)
    .flat_map(|line| line.split(',').flat_map(|v| v.parse()).collect::<Vec<_>>())
    .collect()
}

#[allow(clippy::ptr_arg)]
fn part_1(input: &Vec<i32>) -> i32 {
  let max_val = *input.iter().max().unwrap();
  let mut min_result = i32::MAX;

  // this is kinda brute force'y, prob there are better solutions
  for end_pos in 0..max_val {
    let mut total_fuel_used = 0;
    for crab_pos in input {
      let fuel_used = (crab_pos - end_pos).abs();
      total_fuel_used += fuel_used;
    }
    if total_fuel_used < min_result {
      min_result = total_fuel_used;
    }
  }

  min_result
}

#[allow(clippy::ptr_arg)]
fn part_2(input: &Vec<i32>) -> u64 {
  let max_val = *input.iter().max().unwrap();
  let mut min_result = f64::INFINITY;

  for end_pos in 0..max_val {
    let mut total_fuel_used = 0.0;
    for crab_pos in input {
      let distance = (crab_pos - end_pos).abs() as f64;
      let fuel_used = (1.0 + distance) / 2.0 * distance;
      total_fuel_used += fuel_used;
    }
    if total_fuel_used < min_result {
      min_result = total_fuel_used;
    }
  }

  min_result as u64
}

fn main() {
  aoc2021::run(read_input, part_1, part_2)
}
