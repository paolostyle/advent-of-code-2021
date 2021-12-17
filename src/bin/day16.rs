use itertools::Itertools;

fn read_input() -> String {
  aoc2021::get_input(16).last().unwrap()
}

fn to_bits(input: &str) -> String {
  input
    .chars()
    .map(|char| format!("{:04b}", char.to_digit(16).unwrap()))
    .collect()
}

pub struct Packet {
  version: u64,
  ptype: u64,
  packets: Vec<Packet>,
  value: Option<u64>,
}

impl Packet {
  pub fn new(version: u64, ptype: u64) -> Packet {
    Packet {
      version,
      ptype,
      packets: vec![],
      value: None,
    }
  }

  pub fn sum_versions(&self, sum: &mut u64) {
    *sum += self.version;
    for packet in self.packets.iter() {
      packet.sum_versions(sum);
    }
  }

  pub fn get_result(&mut self) -> u64 {
    if let Some(value) = self.value {
      value
    } else {
      for packet in self.packets.iter_mut() {
        packet.get_result();
      }

      let get_tuple = || {
        self
          .packets
          .iter()
          .flat_map(|p| p.value)
          .collect_tuple::<(u64, u64)>()
          .unwrap()
      };

      self.value = match self.ptype {
        0 => Some(self.packets.iter().flat_map(|p| p.value).sum()),
        1 => Some(self.packets.iter().flat_map(|p| p.value).product()),
        2 => self.packets.iter().flat_map(|p| p.value).min(),
        3 => self.packets.iter().flat_map(|p| p.value).max(),
        4 => self.value,
        5 => {
          let (a, b) = get_tuple();
          Some(u64::from(a > b))
        }
        6 => {
          let (a, b) = get_tuple();
          Some(u64::from(a < b))
        }
        7 => {
          let (a, b) = get_tuple();
          Some(u64::from(a == b))
        }
        _ => panic!("PANIK"),
      };

      self.value.unwrap()
    }
  }
}

pub fn to_decimal(binary: &str) -> u64 {
  u64::from_str_radix(binary, 2).unwrap()
}

fn parse(bits: &mut String) -> Packet {
  let version = to_decimal(&bits[..3]);
  let ptype = to_decimal(&bits[3..6]);
  let mut body = String::from(&bits[6..]);

  let mut packet = Packet::new(version, ptype);

  if ptype == 4 {
    let value = parse_literal(&mut body);
    packet.value = value;
  } else {
    let packets = parse_operator(&mut body);
    packet.packets = packets;
  }

  *bits = body;

  packet
}

fn parse_literal(body: &mut String) -> Option<u64> {
  let mut value = String::new();

  loop {
    let (bits, rest) = body.split_at(5);
    let (control, digit) = bits.split_at(1);
    let control_bit = control.parse::<u8>().unwrap();

    value.push_str(digit);
    *body = rest.to_string();

    if control_bit == 0 {
      break;
    }
  }

  Some(to_decimal(&value))
}

fn parse_operator(body: &mut String) -> Vec<Packet> {
  let lenght_type = &body[..1];
  let mut packets: Vec<Packet> = vec![];

  if lenght_type == "0" {
    let length = to_decimal(&body[1..16]) as usize;
    let mut subpacket_body = String::from(&body[16..(16 + length)]);

    while !subpacket_body.is_empty() {
      let packet = parse(&mut subpacket_body);
      packets.push(packet);
    }

    *body = String::from(&body[(16 + length)..]);
  } else {
    let subpackets = to_decimal(&body[1..12]) as usize;
    let subpacket_body = String::from(&body[12..]);
    let mut counter = 0;
    *body = subpacket_body;

    while counter < subpackets {
      let packet = parse(body);
      packets.push(packet);
      counter += 1;
    }
  }

  packets
}

#[allow(clippy::ptr_arg)]
fn part_1(input: &String) -> u64 {
  let mut bits = to_bits(input);

  let packet = parse(&mut bits);

  let mut version_sum = 0;
  packet.sum_versions(&mut version_sum);

  version_sum
}

#[allow(clippy::ptr_arg)]
fn part_2(input: &String) -> u64 {
  let mut bits = to_bits(input);

  let mut packet = parse(&mut bits);

  packet.get_result()
}

fn main() {
  aoc2021::run(read_input, part_1, part_2)
}
