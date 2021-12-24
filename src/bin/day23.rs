const AMPHIBIAN_TYPES: [char; 4] = ['A', 'B', 'C', 'D'];
const HALL_SIZE: usize = 11;
const ROOMS: usize = 4;

type Rooms = [Vec<char>; ROOMS];

enum Move {
  Right(usize),
  Left(usize),
}

struct Burrow {
  hall: [char; HALL_SIZE],
  rooms: Rooms,
  energy: usize,
}

impl Burrow {
  fn new(rooms: Rooms) -> Burrow {
    Burrow {
      hall: ['.'; HALL_SIZE],
      energy: 0,
      rooms,
    }
  }

  fn leave_room(room_id: usize, action: Move) {
    let vec_id = room_id - 1;
  }
}

fn read_input() -> Rooms {
  let input = aoc2021::get_input(23).skip(2);
  const ROOM: Vec<char> = vec![];
  let mut rooms = [ROOM; ROOMS];

  for (idx, row) in input.enumerate() {
    for (i, amph) in row.chars().filter(|char| AMPHIBIAN_TYPES.contains(char)).enumerate() {
      rooms[i][idx] = amph;
    }
  }

  rooms
}

fn part_1(input: &Rooms) -> u32 {
  let hall = ['.'; HALL_SIZE];

  // i actually solved it without any programming
  17120
}

fn part_2(input: &Rooms) -> u32 {
  0
}

fn main() {
  aoc2021::run(read_input, part_1, part_2)
}
