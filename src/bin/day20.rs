fn read_input() -> (Vec<char>, Vec<Vec<char>>) {
  let mut source = aoc2021::get_input(20);
  let code = source.by_ref().take(1).last().unwrap().chars().collect();
  let map: Vec<Vec<char>> = source
    .skip(1)
    .map(|line| line.chars().collect::<Vec<char>>())
    .collect();

  (code, map)
}

fn enhance((code, map): &(Vec<char>, Vec<Vec<char>>), iterations: usize) -> usize {
  let width = map[0].len();
  let height = map.len();
  let mut padding = 4;
  let mut map_with_margins = vec![vec!['.'; width + padding]; height + padding];

  for (row_index, row) in map.iter().enumerate() {
    for (index, item) in row.iter().enumerate() {
      map_with_margins[row_index + padding / 2][index + padding / 2] = *item;
    }
  }

  for n in 1..=iterations {
    padding += 2 * n;

    let border_fill = if code[0] == '#' && n % 2 == 1 {
      '#'
    } else {
      '.'
    };

    let mut new_map = vec![vec![border_fill; width + padding]; width + padding];

    for (row_index, row) in map_with_margins
      .iter()
      .enumerate()
      .skip(1)
      .take(map_with_margins.len() - 2)
    {
      for (index, _) in row.iter().enumerate().skip(1).take(row.len() - 2) {
        let mut binary = String::new();

        for i in 0..3 {
          for j in 0..3 {
            let val = map_with_margins[row_index + i - 1][index + j - 1];
            let binary_val = match val {
              '.' => '0',
              '#' => '1',
              _ => panic!("wut"),
            };
            binary.push(binary_val);
          }
        }

        let code_index = usize::from_str_radix(&binary, 2).unwrap();
        new_map[row_index + n][index + n] = code[code_index];
      }
    }
    map_with_margins = new_map;
  }

  map_with_margins
    .iter()
    .flatten()
    .filter(|char| **char == '#')
    .count()
}

fn part_1(input: &(Vec<char>, Vec<Vec<char>>)) -> usize {
  enhance(input, 2)
}

fn part_2(input: &(Vec<char>, Vec<Vec<char>>)) -> usize {
  enhance(input, 50)
}

fn main() {
  aoc2021::run(read_input, part_1, part_2)
}
