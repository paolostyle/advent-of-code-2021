use aoc2021::get_input;
use std::{
  cmp::{max, min},
  io::BufRead,
  time::Instant,
};

const MAP_SIZE: usize = 1000;
type Map = [[u16; MAP_SIZE]; MAP_SIZE];

#[derive(Clone, Copy, Debug)]
struct Point {
  x: i16,
  y: i16,
}

impl Point {
  fn new(x: &str, y: &str) -> Point {
    Point {
      x: x.parse::<i16>().expect("Couldn't parse"),
      y: y.parse::<i16>().expect("Couldn't parse"),
    }
  }
}

#[derive(Clone, Debug)]
struct Line {
  a: Point,
  b: Point,
}

impl Line {
  fn new(a: Point, b: Point) -> Line {
    Line { a, b }
  }

  fn is_horizontal(&self) -> bool {
    self.a.x == self.b.x
  }

  fn is_vertical(&self) -> bool {
    self.a.y == self.b.y
  }

  fn is_straight(&self) -> bool {
    self.is_horizontal() || self.is_vertical()
  }

  fn draw(&self, map: &mut Map) {
    if self.is_horizontal() {
      let range = min(self.a.y, self.b.y)..max(self.a.y, self.b.y) + 1;

      for i in range {
        map[i as usize][self.a.x as usize] += 1;
      }
    } else if self.is_vertical() {
      let range = min(self.a.x, self.b.x)..max(self.a.x, self.b.x) + 1;

      for i in range {
        map[self.a.y as usize][i as usize] += 1;
      }
    } else {
      let x_sign = (self.b.x - self.a.x).signum();
      let y_sign = (self.b.y - self.a.y).signum();

      for i in 0..(self.a.x - self.b.x).abs() + 1 {
        let y = (self.a.y + i * y_sign) as usize;
        let x = (self.a.x + i * x_sign) as usize;

        map[y][x] += 1;
      }
    }
  }
}

fn read_input() -> Vec<Line> {
  get_input(5)
    .lines()
    .map(|line| {
      let points: Vec<Point> = line
        .expect("Couldn't read line")
        .split(" -> ")
        .map(|point| {
          let coords: Vec<&str> = point.split(",").collect();
          Point::new(coords[0], coords[1])
        })
        .collect();

      Line::new(points[0], points[1])
    })
    .collect()
}

fn part_1(input: &Vec<Line>) -> usize {
  let straight_lines: Vec<&Line> = input.iter().filter(|line| line.is_straight()).collect();
  let mut map: Map = [[0; MAP_SIZE]; MAP_SIZE];

  for line in straight_lines {
    line.draw(&mut map);
  }

  map.iter().flatten().filter(|val| **val > 1).count()
}

fn part_2(input: &Vec<Line>) -> usize {
  let mut map: Map = [[0; MAP_SIZE]; MAP_SIZE];

  for line in input {
    line.draw(&mut map);
  }

  map.iter().flatten().filter(|val| **val > 1).count()
}

fn main() {
  let now = Instant::now();

  let input = read_input();
  println!("Part 1: {}", part_1(&input));
  println!("Part 2: {}", part_2(&input));

  let elapsed = now.elapsed();
  println!("Elapsed: {:.2?}", elapsed);
}
