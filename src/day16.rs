use std::str::FromStr;
use bitreader::{BitReader, BitReaderError};
use num_derive::FromPrimitive;
use num_traits::FromPrimitive;

const INPUT : &'static str = include_str!("../inputs/day16.txt");

#[derive(Eq, PartialEq, Clone, Copy, Debug, FromPrimitive, ToPrimitive)]
pub enum Operator {
  Sum     = 0,
  Product = 1,
  Minimum = 2,
  Maximum = 3,
  Greater = 5,
  Lesser  = 6,
  Equals  = 7
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Packet {
  Literal { version: u8, value: u64 },
  Operator { version: u8, operator: Operator, children: Vec<Packet> },
}

impl FromStr for Packet {
  type Err = &'static str;

  fn from_str(s: &str) -> Result<Self, Self::Err> {
    let input = s.trim();

    if input.len() & 1 == 1 {
      return Err("Odd number of characters");
    }

    let bytes : Vec<u8> = (0..input.len())
      .step_by(2)
      .map(|i| u8::from_str_radix(&input[i..(i+2)], 16).map_err(|_| "Failed to parse digit"))
      .collect::<Result<Vec<u8>, Self::Err>>()?;

    let mut reader = BitReader::new(bytes.as_ref());
    Packet::from_reader(&mut reader).map_err(|_| "BitReader error")
  }
}

impl Packet {
  pub fn version_sum(&self) -> usize {
    match self {
      Packet::Literal { version, .. } => *version as usize,
      Packet::Operator { version, children, .. } => {
        children.iter().map(|c| c.version_sum()).sum::<usize>() + (*version as usize)
      }
    }
  }

  pub fn value(&self) -> u64 {
    match self {
      Packet::Literal { value, .. } => *value,
      Packet::Operator { operator, children, .. } => {
        let mut v = children.iter().map(|c| c.value());
        match operator {
          Operator::Sum     => v.sum(),
          Operator::Product => v.product(),
          Operator::Minimum => v.min().unwrap(),
          Operator::Maximum => v.max().unwrap(),
          Operator::Greater => if v.next().unwrap() > v.next().unwrap() { 1 } else { 0 },
          Operator::Lesser  => if v.next().unwrap() < v.next().unwrap() { 1 } else { 0 },
          Operator::Equals  => if v.next().unwrap() == v.next().unwrap() { 1 } else { 0 },
        }
      }
    }
  }

  pub fn from_reader(reader: &mut BitReader) -> Result<Packet, BitReaderError> {
    let version = reader.read_u8(3)?;

    match reader.read_u8(3)? {
      4 => {
        let mut value : u64 = 0;
        while reader.read_bool()? {
          value = (value << 4) | reader.read_u64(4)?;
        }
        value = (value << 4) | reader.read_u64(4)?;

        Ok(Packet::Literal { version, value })
      },
      op if op < 8 => Ok(Packet::Operator {
        version,
        operator: Operator::from_u8(op).unwrap(),
        children: Packet::read_children(reader)?
      }),
      op => panic!("Unhandled packet type: {}", op)
    }
  }

  fn read_children(reader: &mut BitReader) -> Result<Vec<Packet>, BitReaderError> {
    if reader.read_bool()? {
      // 11 bit count
      let count = reader.read_u16(11)?;
      (0..count)
        .map(|_| Packet::from_reader(reader))
        .collect::<Result<Vec<Packet>, _>>()
    } else {
      // 15 bit len
      let len = reader.read_u16(15)? as u64;
      let end = reader.position() + len;
      let mut children = vec![];

      while reader.position() < end {
        children.push(Packet::from_reader(reader)?);
      }

      Ok(children)
    }
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_decode() {
    assert_eq!("D2FE28".parse(), Ok(Packet::Literal { version: 6, value: 2021 }));
    assert_eq!("EE00D40C823060".parse(), Ok(Packet::Operator {
      version: 7,
      operator: Operator::Maximum,
      children: vec![
        Packet::Literal { version: 2, value: 1 },
        Packet::Literal { version: 4, value: 2 },
        Packet::Literal { version: 1, value: 3 },
      ]
    }));
    assert_eq!("38006F45291200".parse(), Ok(Packet::Operator {
      version: 1,
      operator: Operator::Lesser,
      children: vec![
        Packet::Literal { version: 6, value: 10 },
        Packet::Literal { version: 2, value: 20 },
      ]
    }));
  }

  #[test]
  fn part1_example() {
    let p : Packet = "8A004A801A8002F478".parse().expect("shit");
    assert_eq!(p.version_sum(), 16);
  }

  #[test]
  fn part1_solution() {
    let p : Packet = INPUT.parse().expect("shit");
    assert_eq!(p.version_sum(), 904);
  }

  #[test]
  fn part2_solution() {
    let p : Packet = INPUT.parse().expect("shit");
    assert_eq!(p.value(), 200476472872);
  }
}
