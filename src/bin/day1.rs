fn read_input() -> Vec<u16> {
  aoc2021::get_input(1)
    .flat_map(|line| line.parse())
    .collect()
}

#[allow(clippy::ptr_arg)]
fn part_1(input: &Vec<u16>) -> u16 {
  let mut increases: u16 = 0;

  for (index, value) in input.iter().enumerate() {
    if index > 0 {
      let prev_value = input[index - 1];
      if value > &prev_value {
        increases += 1;
      }
    }
  }

  increases
}

#[allow(clippy::ptr_arg)]
fn part_2(input: &Vec<u16>) -> u16 {
  let mut increases: u16 = 0;

  // far from optimal
  for i in 0..(input.len() - 3) {
    let window_1_sum: u16 = input[i..i + 3].iter().sum();
    let window_2_sum: u16 = input[i + 1..i + 4].iter().sum();

    if window_2_sum > window_1_sum {
      increases += 1
    }
  }

  increases
}

fn main() {
  aoc2021::run(read_input, part_1, part_2)
}
