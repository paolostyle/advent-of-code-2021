use itertools::Itertools;
use petgraph::{graph::NodeIndex, visit::Dfs, Graph};
use std::collections::{HashMap, HashSet};

fn read_input() -> Vec<Vec<u32>> {
  aoc2021::get_input(9)
    .map(|line| {
      line
        .chars()
        .map(|height| height.to_digit(10).expect("Could not parse digit"))
        .collect()
    })
    .collect()
}

#[allow(clippy::ptr_arg)]
fn part_1(input: &Vec<Vec<u32>>) -> u32 {
  let mut risk_levels = 0;

  let lines = input.len();
  let cols = input[0].len();

  for (i, line) in input.iter().enumerate() {
    for (j, height) in line.iter().enumerate() {
      let up_lower = i > 0 && *height >= input[i - 1][j];
      let down_lower = i + 1 < lines && *height >= input[i + 1][j];
      let left_lower = j > 0 && *height >= input[i][j - 1];
      let right_lower = j + 1 < cols && *height >= input[i][j + 1];

      if up_lower || down_lower || left_lower || right_lower {
        continue;
      }

      risk_levels += height + 1;
    }
  }

  risk_levels
}

#[allow(clippy::ptr_arg)]
fn part_2(input: &Vec<Vec<u32>>) -> u32 {
  let lines = input.len();
  let cols = input[0].len();

  let mut graph = Graph::new_undirected();
  let mut nodes = HashMap::with_capacity(lines * cols);

  for (node_id, weight) in input.iter().flatten().enumerate() {
    nodes.insert(node_id, graph.add_node(*weight));
  }

  for (node_id, weight) in input.iter().flatten().enumerate() {
    let node = nodes[&node_id];

    let mut neighbouring_node_ids: Vec<usize> = Vec::with_capacity(4);
    if node_id >= cols {
      neighbouring_node_ids.push(node_id - cols);
    }
    if node_id < (lines - 1) * cols {
      neighbouring_node_ids.push(node_id + cols);
    }
    if node_id % cols != 0 {
      neighbouring_node_ids.push(node_id - 1);
    }
    if node_id % cols != cols - 1 {
      neighbouring_node_ids.push(node_id + 1);
    }

    for neighbour in neighbouring_node_ids {
      let neighbour_node = nodes[&neighbour];
      let neighbour_value = graph[neighbour_node];

      if *weight != 9 && neighbour_value != 9 {
        graph.add_edge(node, neighbour_node, 1);
      }
    }
  }

  let mut visited_nodes: HashSet<NodeIndex> = HashSet::new();
  let mut basins: Vec<u32> = Vec::new();

  for node in graph.node_indices() {
    if visited_nodes.contains(&node) {
      continue;
    }

    let mut node_count = 0;

    let mut dfs = Dfs::new(&graph, node);
    while let Some(node) = dfs.next(&graph) {
      node_count += 1;
      visited_nodes.insert(node);
    }

    basins.push(node_count);
  }

  basins.iter().sorted_unstable().rev().take(3).product()
}

fn main() {
  aoc2021::run(read_input, part_1, part_2)
}
