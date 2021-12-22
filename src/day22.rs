use std::collections::HashSet;
use std::ops::RangeInclusive;
use std::str::FromStr;
use regex::Regex;

const INPUT : &'static str = include_str!("../inputs/day22.txt");
const SAMPLE : &'static str = include_str!("../inputs/day22.sample.txt");

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
struct Cube {
  x: (isize, isize),
  y: (isize, isize),
  z: (isize, isize),
}

type IRange = RangeInclusive<isize>;

impl Cube {
  fn cube(min: isize, max: isize) -> Self {
    Self { x: (min, max), y: (min, max), z: (min, max) }
  }

  fn turn_off_region(&self, region: &Cube) -> Vec<Cube> {
    let mut results = vec![];

    // does not intersect, return original cube
    if !self.intersects(region) {
      results.push(*self);
      return results;
    }

    // completely contained, return empty set
    if region.contains(self) {
      return results;
    }
    // println!("breaking {:?}", self);

    // top core piece
    results.push(Cube {
      x: ((region.x.0.max(self.x.0), region.x.1.min(self.x.1))),
      y: (region.y.1 + 1, self.y.1),
      z: ((region.z.0.max(self.z.0), region.z.1.min(self.z.1))),
    });

    // bottom core piece
    results.push(Cube {
      x: ((region.x.0.max(self.x.0), region.x.1.min(self.x.1))),
      y: (self.y.0, region.y.0 - 1),
      z: ((region.z.0.max(self.z.0), region.z.1.min(self.z.1))),
    });

    // left hand piece
    results.push(Cube {
      x: (self.x.0, region.x.0 - 1),
      y: self.y,
      z: self.z
    });

    // right hand piece
    results.push(Cube {
      x: (region.x.1 + 1, self.x.1),
      y: self.y,
      z: self.z
    });

    // front piece
    results.push(Cube {
      x: ((region.x.0.max(self.x.0), region.x.1.min(self.x.1))),
      y: self.y,
      z: (self.z.0, region.z.0 - 1)
    });

    // back piece
    results.push(Cube {
      x: ((region.x.0.max(self.x.0), region.x.1.min(self.x.1))),
      y: self.y,
      z: (region.z.1 + 1, self.z.1)
    });

    results.retain(|cube| !cube.is_empty());
    results
  }

  fn is_empty(&self) -> bool {
    self.x.0 > self.x.1 || self.y.0 > self.y.1 || self.z.0 > self.z.1
  }

  fn contains(&self, other: &Cube) -> bool {
    self.x.0 <= other.x.0 && self.x.1 >= other.x.1 &&
    self.y.0 <= other.y.0 && self.y.1 >= other.y.1 &&
    self.z.0 <= other.z.0 && self.z.1 >= other.z.1
  }

  fn intersects(&self, other: &Cube) -> bool {
    ((self.x.0 <= other.x.0 && other.x.0 <= self.x.1) || (other.x.0 <= self.x.0 && self.x.0 <= other.x.1)) &&
      ((self.y.0 <= other.y.0 && other.y.0 <= self.y.1) || (other.y.0 <= self.y.0 && self.y.0 <= other.y.1)) &&
      ((self.z.0 <= other.z.0 && other.z.0 <= self.z.1) || (other.z.0 <= self.z.0 && self.z.0 <= other.z.1))
  }

  fn volume(&self) -> isize {
    (self.x.1 - self.x.0 + 1) * (self.y.1 - self.y.0 + 1) * (self.z.1 - self.z.0 + 1)
  }
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
struct Instruction {
  on: bool,
  cube: Cube
}

#[derive(Debug, Clone)]
struct Reactor {
  instructions: Vec<Instruction>,
  cubes: HashSet<Cube>
}

impl FromStr for Instruction {
  type Err = &'static str;

  fn from_str(s: &str) -> Result<Self, Self::Err> {
    lazy_static! {
      static ref RES : &'static str = r#"(on|off) x=(-?\d+)\.\.(-?\d+),y=(-?\d+)\.\.(-?\d+),z=(-?\d+)\.\.(-?\d+)"#;
      static ref RE : Regex = Regex::new(&RES).unwrap();
    }

    let captures = RE.captures(s).ok_or("Failed to match input")?;
    let values = captures
      .iter()
      .skip(2)
      .map(|m| m.unwrap().as_str().parse::<isize>().unwrap())
      .collect::<Vec<isize>>();

    let x = (values[0], values[1]);
    let y = (values[2], values[3]);
    let z = (values[4], values[5]);
    let cube = Cube { x, y, z };

    let i = match captures.get(1).unwrap().as_str() {
      "on"  => Instruction { on: true, cube },
      "off" => Instruction { on: false, cube },
      _ => unreachable!()
    };

    Ok(i)
  }
}

impl FromStr for Reactor {
  type Err = &'static str;

  fn from_str(s: &str) -> Result<Self, Self::Err> {
    s
      .trim()
      .lines()
      .map(|line| line.parse::<Instruction>())
      .collect::<Result<Vec<Instruction>, Self::Err>>()
      .map(|instructions| Self { instructions, cubes: HashSet::new() })
  }
}

impl Reactor {
  pub fn clip(&mut self, range: Cube) {
    self.instructions.retain(|i| range.contains(&i.cube));
  }

  pub fn initialize(&mut self) {
    for i in 0..self.instructions.len() {
      let instr = self.instructions[i];
      if instr.on {
        self.turn_on(&instr.cube);
      } else {
        self.turn_off(&instr.cube);
      }
    }
  }

  pub fn turn_on(&mut self, cube: &Cube) {
    // println!("turn on {:?}", cube);
    self.turn_off(cube);
    self.cubes.insert(*cube);
  }

  pub fn turn_off(&mut self, cube: &Cube) {
    // println!("turn off {:?}", cube);
    self.cubes = self.cubes
      .iter()
      .flat_map(|c| c.turn_off_region(cube))
      .collect();
  }

  fn count(&self) -> usize {
    self.cubes.iter().map(|cube| cube.volume() as usize).sum()
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn part1_example() {
    let mut reactor : Reactor = SAMPLE.parse().expect("shit");
    reactor.clip(Cube::cube(-50, 50));
    reactor.initialize();
    assert_eq!(reactor.count(), 39);
  }

  #[test]
  fn part1_solution() {
    let mut reactor : Reactor = INPUT.parse().expect("shit");
    reactor.clip(Cube::cube(-50, 50));
    reactor.initialize();
    assert_eq!(reactor.count(), 546724);
  }

  #[test]
  fn test_intersect() {
    // c1 contains c2
    let c1 = Cube { x: (-1, 1), y: (-1, 1), z: (-1, 1) };
    let c2 = Cube { x: (-2, 2), y: (-2, 2), z: (-2, 2) };
    let c3 = Cube { x: (2, 3),  y: (2, 3), z: (2, 3) };
    assert!(c1.intersects(&c2));
    assert!(c2.intersects(&c1));
    assert!(!c1.intersects(&c3));
    assert!(c2.intersects(&c3));
  }

  #[test]
  fn part2_example() {

  }

  #[test]
  fn part2_solution() {
    let mut reactor : Reactor = INPUT.parse().expect("shit");
    reactor.initialize();
    assert_eq!(reactor.count(), 1346544039176841);
  }
}
