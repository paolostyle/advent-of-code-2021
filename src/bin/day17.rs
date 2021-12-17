use std::cmp;

use itertools::Itertools;
use regex::Regex;

type Goal = ((i32, i32), (i32, i32));

fn read_input() -> Goal {
  let string = aoc2021::get_input(17).last().unwrap();
  let re = Regex::new(r"target area: x=([\d-]+)..([\d-]+), y=([\d-]+)..([\d-]+)").unwrap();
  let matches = re.captures(&string).unwrap();

  let mut out = [0; 4];

  for i in 1..5 {
    out[i - 1] = matches[i].parse::<i32>().unwrap();
  }

  ((out[0], out[1]), (out[2], out[3]))
}

fn part_1(input: &Goal) -> i32 {
  let (_, (y1, y2)) = *input;
  let mut valid_y: Vec<i32> = vec![];
  let mut max_height = i32::MIN;

  for y in y1..-y1 {
    let mut step = 1;
    let mut py = 0;
    let mut local_max = i32::MIN;
    loop {
      py += y - (step - 1);
      step += 1;
      if py > local_max {
        local_max = py;
      }

      if (y1..=y2).contains(&py) {
        valid_y.push(y);
        if local_max > max_height {
          max_height = local_max;
        }
        break;
      } else if py < y1 {
        break;
      }
    }
  }

  max_height
}

fn part_2(input: &Goal) -> usize {
  let ((x1, x2), (y1, y2)) = *input;
  let mut valid_x: Vec<i32> = (x1..=x2).into_iter().collect();
  let mut valid_y: Vec<i32> = vec![];

  for x in 0..=((x2 + 1) / 2) {
    let mut step = 1;
    let mut px = 0;
    loop {
      let px_inc = cmp::max(x - (step - 1), 0);
      px += px_inc;
      step += 1;

      if (x1..=x2).contains(&px) {
        valid_x.push(x);
        break;
      } else if px > x2 || px_inc == 0 {
        break;
      }
    }
  }

  for y in y1..-y1 {
    let mut step = 1;
    let mut py = 0;
    loop {
      let py_inc = y - (step - 1);
      py += py_inc;
      step += 1;

      if (y1..=y2).contains(&py) {
        valid_y.push(y);
        break;
      } else if py < y1 {
        break;
      }
    }
  }

  let mut valid_pairs = vec![];

  for (x, y) in valid_x.iter().cartesian_product(valid_y) {
    let mut step = 1;
    let mut px = 0;
    let mut py = 0;
    loop {
      px += cmp::max(x - (step - 1), 0);
      py += y - (step - 1);
      step += 1;

      if (x1..=x2).contains(&px) && (y1..=y2).contains(&py) {
        valid_pairs.push((x, y));
        break;
      } else if px > x2 || py < y1 {
        break;
      }
    }
  }

  valid_pairs.len()
}

fn main() {
  aoc2021::run(read_input, part_1, part_2)
}
