use std::{
  fs::File,
  io::BufReader,
};

pub fn get_input(day: u8) -> BufReader<File> {
  let input_file = File::open(format!("inputs/day{}.txt", day)).expect("Input file doesn't exist");
  BufReader::new(input_file)
}
