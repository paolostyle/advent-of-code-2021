#!/bin/sh
curl "https://adventofcode.com/2021/day/$1/input" --cookie "session=$(cat .session)" > "inputs/day$1.txt"
cat > "src/bin/day$1.rs" << EOF
use aoc2021::get_input;
use std::{io::BufRead, time::Instant};

type Data = String;

fn read_input() -> Vec<Data> {
  get_input($1).lines().flatten().collect()
}

fn part_1(input: &Vec<Data>) -> u32 {
  0 
}

fn part_2(input: &Vec<Data>) -> u32 {
  0 
}

fn main() {
  let now = Instant::now();

  let input = read_input();
  println!("Part 1: {}", part_1(&input));
  println!("Part 2: {}", part_2(&input));

  let elapsed = now.elapsed();
  println!("Elapsed: {:.2?}", elapsed);
}
EOF
