fn read_input() -> Vec<Vec<char>> {
  aoc2021::get_input(25)
    .map(|line| line.chars().collect())
    .collect()
}

fn move_cucumbers(map: &[Vec<char>]) -> (Vec<Vec<char>>, bool) {
  let mut new_map = map.to_vec();

  for (row_idx, row) in map.iter().enumerate() {
    for (idx, item) in row.iter().enumerate() {
      let new_idx = (idx + 1) % row.len();
      if *item == '>' && row[new_idx] == '.' {
        new_map[row_idx][idx] = '.';
        new_map[row_idx][new_idx] = '>';
      }
    }
  }

  let intermediate_state = new_map.clone();

  for (row_idx, row) in map.iter().enumerate() {
    for (idx, item) in row.iter().enumerate() {
      let new_row_idx = (row_idx + 1) % map.len();
      if *item == 'v' && intermediate_state[new_row_idx][idx] == '.' {
        new_map[row_idx][idx] = '.';
        new_map[new_row_idx][idx] = 'v';
      }
    }
  }

  let stopped_moving = new_map.iter().enumerate().all(|(row_idx, row)| {
    row
      .iter()
      .enumerate()
      .all(|(idx, item)| *item == map[row_idx][idx])
  });

  (new_map, stopped_moving)
}

#[allow(clippy::ptr_arg)]
fn part_1(input: &Vec<Vec<char>>) -> usize {
  let mut step = 0;
  let mut map: Vec<Vec<char>> = input.to_vec();

  loop {
    let (new_map, stopped_moving) = move_cucumbers(&map);

    step += 1;

    if stopped_moving {
      break step;
    } else {
      map = new_map;
    }
  }
}

#[allow(clippy::ptr_arg)]
fn part_2(_input: &Vec<Vec<char>>) -> &'static str {
  "not enough stars yet :("
}

fn main() {
  aoc2021::run(read_input, part_1, part_2)
}
