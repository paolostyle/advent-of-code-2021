use std::{fs::File, io::BufReader};

fn load_file(path: String) -> BufReader<File> {
  let input_file = File::open(path).expect("Input file doesn't exist");
  BufReader::new(input_file)
}

pub fn get_input(day: u8) -> BufReader<File> {
  load_file(format!("inputs/day{}.txt", day))
}

pub fn get_test_input(day: u8) -> BufReader<File> {
  load_file(format!("test_inputs/day{}.txt", day))
}
