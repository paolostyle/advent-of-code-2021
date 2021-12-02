use std::{
    fs::File,
    io::{prelude::*, BufReader},
};

fn read_input() -> Vec<u16> {
    let input_file = File::open("inputs/day1.txt").expect("Input file doesn't exist");
    let buffer = BufReader::new(input_file);
    buffer
        .lines()
        .map(|line| {
            line.expect("could not read line")
                .parse::<u16>()
                .expect("could not parse string")
        })
        .collect()
}

fn part_1() -> u16 {
    let data = read_input();
    let mut increases: u16 = 0;

    for (index, value) in data.iter().enumerate() {
        if index > 0 {
            let prev_value = data[index - 1];
            if value > &prev_value {
                increases += 1;
            }
        }
    }

    increases
}

fn part_2() -> u16 {
    let data = read_input();
    let mut increases: u16 = 0;

    // far from optimal
    for i in 0..(data.len() - 3) {
        let window_1_sum: u16 = data[i..i+3].iter().sum();
        let window_2_sum: u16 = data[i+1..i+4].iter().sum();

        if window_2_sum > window_1_sum {
            increases += 1
        }
    }

    increases
}

fn main() {
    println!("Part 1: {}", part_1());
    println!("Task 2: {}", part_2());
}
