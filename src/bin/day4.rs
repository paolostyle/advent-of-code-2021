use aoc2021::get_input;
use std::io::*;

const BOARD_SIZE: usize = 5;

#[derive(Clone, Debug)]
struct BingoBoard {
  entries: Vec<Vec<u8>>,
  marks: Vec<Vec<bool>>,
}

impl BingoBoard {
  fn new() -> BingoBoard {
    BingoBoard {
      entries: Vec::with_capacity(BOARD_SIZE),
      marks: vec![vec![false; BOARD_SIZE]; BOARD_SIZE],
    }
  }

  fn is_empty(&mut self) -> bool {
    self.entries.is_empty()
  }

  fn push_entry(&mut self, entries: Vec<u8>) {
    self.entries.push(entries)
  }

  fn mark(&mut self, draw: &u8) {
    let maybe_match = self
      .entries
      .iter()
      .flatten()
      .enumerate()
      .find(|(_, val)| (*val).eq(draw));

    if let Some((index, _)) = maybe_match {
      self.marks[index / BOARD_SIZE][index % BOARD_SIZE] = true;
    }
  }

  fn is_winning(&self) -> bool {
    let row_win = self.marks.iter().any(|row| row.iter().all(|x| *x));
    let col_win = (0..BOARD_SIZE).any(|col_number| self.marks.iter().all(|row| row[col_number]));

    row_win || col_win
  }

  fn get_sum(&self) -> u32 {
    self
      .entries
      .iter()
      .flatten()
      .enumerate()
      .filter(|(i, _)| !self.marks[i / BOARD_SIZE][i % BOARD_SIZE])
      .map(|(_, val)| *val as u32)
      .sum()
  }
}

fn read_input() -> (Vec<u8>, Vec<BingoBoard>) {
  let mut lines = get_input(4).lines();
  let draws = lines
    .next()
    .expect("Couldn't read draws")
    .unwrap()
    .split(",")
    .map(|draw| draw.parse::<u8>().expect("Couldn't parse draw"))
    .collect();

  let mut boards: Vec<BingoBoard> = Vec::new();
  let mut board: Option<BingoBoard> = None;

  for line in lines {
    let read_line = line.expect("Couldn't read line");
    if read_line.is_empty() {
      match &board {
        None => board = Some(BingoBoard::new()),
        Some(b) => {
          boards.push(b.clone());
          board = Some(BingoBoard::new());
        }
      }
    } else {
      let number_line = read_line
        .split_whitespace()
        .map(|num| num.parse::<u8>().expect("Couldn't parse number"))
        .collect();

      if let Some(b) = &mut board {
        b.push_entry(number_line);
      }
    }
  }

  if let Some(b) = &mut board {
    if !b.is_empty() {
      boards.push(b.clone())
    }
  }

  (draws, boards)
}

fn part_1() -> u32 {
  let (draws, mut boards) = read_input();
  let mut winning_board: usize = 0;
  let mut draws_iter = draws.iter();
  let mut draw: &u8 = &0;

  while winning_board == 0 {
    draw = draws_iter.next().unwrap();

    for (index, board) in boards.iter_mut().enumerate() {
      board.mark(draw);
      if board.is_winning() {
        winning_board = index;
        break;
      }
    }
  }

  boards[winning_board].get_sum() * ((*draw) as u32)
}

fn part_2() -> u32 {
  let (draws, mut boards) = read_input();
  let mut draws_iter = draws.iter();
  let mut draw: &u8 = &0;
  let mut last_board: Option<BingoBoard> = None;

  // this is awful ngl
  while boards.len() > 0 {
    draw = draws_iter.next().unwrap();
    let mut boards_to_keep = vec![true; boards.len()];

    for (index, board) in boards.iter_mut().enumerate() {
      board.mark(draw);
      if board.is_winning() {
        boards_to_keep[index] = false;
      }
    }

    let mut iter = boards_to_keep.iter();
    boards.retain(|board| {
      let should_keep = *iter.next().unwrap();
      if !should_keep {
        last_board = Some(board.clone());
      }
      should_keep
    });
  }

  if let Some(board) = last_board {
    board.get_sum() * ((*draw) as u32)
  } else {
    panic!("no board")
  }
}

fn main() {
  println!("Part 1: {}", part_1());
  println!("Part 2: {}", part_2());
}
