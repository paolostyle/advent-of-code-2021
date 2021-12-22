use std::cmp;

use cached::proc_macro::cached;
use itertools::Itertools;

fn read_input() -> (usize, usize) {
  aoc2021::get_input(21)
    .map(|line| line.split(": ").last().unwrap().parse::<usize>().unwrap())
    .collect_tuple()
    .unwrap()
}

fn next(current: usize, step: usize, range: usize) -> usize {
  (current + step - 1) % range + 1
}

fn part_1((p1, p2): &(usize, usize)) -> usize {
  let mut p1_position = *p1;
  let mut p1_points = 0;
  let mut p2_position = *p2;
  let mut p2_points = 0;
  let mut rolls = 0;
  let mut dice_state = 1;
  let mut p1_turn = true;

  while p1_points < 1000 && p2_points < 1000 {
    let rolls_sum = dice_state + next(dice_state, 1, 100) + next(dice_state, 2, 100);
    dice_state = next(dice_state, 3, 100);

    if p1_turn {
      p1_position = next(p1_position, rolls_sum, 10);
      p1_points += p1_position;
      p1_turn = false;
    } else {
      p2_position = next(p2_position, rolls_sum, 10);
      p2_points += p2_position;
      p1_turn = true;
    }

    rolls += 3;
  }

  rolls * cmp::min(p1_points, p2_points)
}

const ROLLS_SUMS: [(usize, usize); 7] = [(3, 1), (4, 3), (5, 6), (6, 7), (7, 6), (8, 3), (9, 1)];

#[cached]
fn dirac_turn(
  curr_points: usize,
  curr_position: usize,
  other_points: usize,
  other_position: usize,
) -> (usize, usize) {
  if curr_points >= 21 {
    return (1, 0);
  }
  if other_points >= 21 {
    return (0, 1);
  }

  let mut wins = (0, 0);

  for (pos, count) in ROLLS_SUMS {
    let new_pos = next(curr_position, pos, 10);

    let (other_wins, player_wins) =
      dirac_turn(other_points, other_position, curr_points + new_pos, new_pos);

    wins.0 += player_wins * count;
    wins.1 += other_wins * count;
  }

  wins
}

fn part_2((p1, p2): &(usize, usize)) -> usize {
  let (p1_wins, p2_wins) = dirac_turn(0, *p1, 0, *p2);

  cmp::max(p1_wins, p2_wins)
}

fn main() {
  aoc2021::run(read_input, part_1, part_2)
}
