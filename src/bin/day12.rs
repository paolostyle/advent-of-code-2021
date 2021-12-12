use itertools::Itertools;
use petgraph::graphmap::UnGraphMap;

fn read_input() -> Vec<(String, String)> {
  aoc2021::get_input(12)
    .flat_map(|pair| pair.split('-').map_into::<String>().collect_tuple())
    .collect()
}

fn create_graph(input: &[(String, String)]) -> UnGraphMap<&str, ()> {
  let edges: Vec<(&str, &str)> = input
    .iter()
    .map(|(a, b)| (a.as_str(), b.as_str()))
    .collect();

  UnGraphMap::<_, ()>::from_edges(edges)
}

fn is_big_cave(node: &str) -> bool {
  node != node.to_lowercase()
}

#[allow(clippy::ptr_arg)]
fn part_1(input: &Vec<(String, String)>) -> usize {
  let graph = create_graph(input);

  let mut paths_found = 0;
  let mut queue: Vec<Vec<&str>> = vec![vec!["start"]];

  while !queue.is_empty() {
    let path = queue.pop().unwrap();
    let last_node = path.last().unwrap();

    if *last_node == "end" {
      paths_found += 1;
      continue;
    }

    for node in graph.neighbors(last_node) {
      if is_big_cave(node) || !path.contains(&node) {
        let mut new_path = path.clone();
        new_path.push(node);
        queue.push(new_path);
      }
    }
  }

  paths_found
}

#[allow(clippy::ptr_arg)]
fn part_2(input: &Vec<(String, String)>) -> u32 {
  let graph = create_graph(input);

  let mut paths_found = 0;
  let mut queue: Vec<Vec<&str>> = vec![vec!["start"]];

  while !queue.is_empty() {
    let path = queue.pop().unwrap();
    let counts = path.iter().counts();
    let last_node = path.last().unwrap();

    if *last_node == "end" {
      paths_found += 1;
      continue;
    }

    for node in graph.neighbors(last_node) {
      if node == "start" {
        continue;
      }

      let mut can_visit = is_big_cave(node) || node == "end" || !counts.contains_key(&node);
      if !can_visit {
        let any_small_cave_visited_twice = counts
          .iter()
          .filter(|(key, _)| !is_big_cave(key))
          .any(|(_, value)| *value == 2);

        can_visit = !any_small_cave_visited_twice
      }

      if can_visit {
        let mut new_path = path.clone();
        new_path.push(node);
        queue.push(new_path);
      }
    }
  }

  paths_found
}

fn main() {
  aoc2021::run(read_input, part_1, part_2)
}
