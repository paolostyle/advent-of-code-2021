#!/bin/sh
curl "https://adventofcode.com/2021/day/$1/input" --cookie "session=$(cat .session)" > "inputs/day$1.txt"
cat > "src/bin/day$1.rs" << EOF
fn read_input() -> Vec<String> {
  aoc2021::get_input($1).collect()
}

#[allow(clippy::ptr_arg)]
fn part_1(input: &Vec<String>) -> u32 {
  0 
}

#[allow(clippy::ptr_arg)]
fn part_2(input: &Vec<String>) -> u32 {
  0 
}

fn main() {
  aoc2021::run(read_input, part_1, part_2)
}
EOF
