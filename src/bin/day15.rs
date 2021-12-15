use petgraph::{algo::dijkstra, graphmap::DiGraphMap};

fn read_input() -> Vec<Vec<u32>> {
  aoc2021::get_input(15)
    .map(|line| {
      line
        .chars()
        .flat_map(|char| char.to_digit(10))
        .collect::<Vec<u32>>()
    })
    .collect()
}

fn find_path_cost(input: &[Vec<u32>]) -> u32 {
  let width = input[0].len();
  let height = input.len();
  let mut map_with_margins = vec![vec![0; width + 2]; height + 2];

  for (row_index, row) in input.iter().enumerate() {
    for (index, item) in row.iter().enumerate() {
      map_with_margins[row_index + 1][index + 1] = *item;
    }
  }

  let mut edges = vec![];

  for (row_index, row) in map_with_margins.iter().enumerate().skip(1).take(height) {
    for (index, _) in row.iter().enumerate().skip(1).take(height) {
      let idx = row_index * width + index;
      let neighbours = [
        (idx, idx - width, map_with_margins[row_index - 1][index]),
        (idx, idx + width, map_with_margins[row_index + 1][index]),
        (idx, idx - 1, map_with_margins[row_index][index - 1]),
        (idx, idx + 1, map_with_margins[row_index][index + 1]),
      ];

      for (idx_1, idx_2, weight) in neighbours {
        if weight != 0 {
          edges.push((idx_1 - width - 1, idx_2 - width - 1, weight));
        }
      }
    }
  }

  let graph = DiGraphMap::<usize, u32>::from_edges(&edges);
  // meh
  let path = dijkstra(&graph, 0, Some(width * height - 1), |e| *e.2);

  path[&(width * height - 1)]
}

fn new_tile(add: u32, initial_input: &[Vec<u32>]) -> Vec<Vec<u32>> {
  initial_input
    .iter()
    .map(|row| {
      row
        .iter()
        .map(|risk| {
          let new_value = *risk + add;

          if new_value > 9 {
            new_value - 9
          } else {
            new_value
          }
        })
        .collect()
    })
    .collect()
}

#[allow(clippy::ptr_arg)]
fn part_1(input: &Vec<Vec<u32>>) -> u32 {
  find_path_cost(input)
}

#[allow(clippy::ptr_arg)]
fn part_2(input: &Vec<Vec<u32>>) -> u32 {
  let mut big_map = input.clone();

  for i in 1..5 {
    let tile = new_tile(i, input);
    for (idx, row) in big_map.iter_mut().enumerate() {
      row.extend(tile[idx].iter())
    }
  }

  let extended_tile = big_map.clone();

  for i in 1..5 {
    let tile = new_tile(i, &extended_tile);
    big_map.extend(tile);
  }

  find_path_cost(&big_map)
}

fn main() {
  aoc2021::run(read_input, part_1, part_2)
}
