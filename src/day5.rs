use std::cmp::Ordering;
use std::collections::HashSet;
use std::str::FromStr;
use std::error::Error;
use std::io::{Error as IoError, ErrorKind};

const INPUT : &'static str = include_str!("../inputs/day5.txt");
const SAMPLE : &'static str = include_str!("../inputs/day5.sample.txt");

fn input(s: &'static str) -> Result<Vec<Line>, Box<dyn Error>> {
  s
    .lines()
    .map(|v| v.parse())
    .collect()
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
struct Point {
  x: isize,
  y: isize,
}

impl FromStr for Point {
  type Err = Box<dyn Error>;

  fn from_str(s: &str) -> Result<Self, Self::Err> {
    let mut parts = s.split(",").map(|v| v.parse());
    let x = parts
      .next()
      .ok_or(IoError::new(ErrorKind::UnexpectedEof, "No X value"))??;
    let y = parts
      .next()
      .ok_or(IoError::new(ErrorKind::UnexpectedEof, "No Y value"))??;

    Ok(Self { x, y })
  }
}

#[derive(Debug, Copy, Clone)]
struct Line {
  p1: Point,
  p2: Point
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum Orientation {
  Horizontal,
  Vertical,
  Diagonal
}

impl FromStr for Line {
  type Err = Box<dyn Error>;

  fn from_str(s: &str) -> Result<Self, Self::Err> {
    let mut parts = s.split(" -> ").map(|v| v.parse());
    let p1 = parts
      .next()
      .ok_or(IoError::new(ErrorKind::UnexpectedEof, "No P1 value"))??;
    let p2 = parts
      .next()
      .ok_or(IoError::new(ErrorKind::UnexpectedEof, "No P2 value"))??;

    Ok(Self { p1, p2 })
  }
}

impl Line {
  pub fn orientation(&self) -> Orientation {
    match (self.p1.x.cmp(&self.p2.x), self.p1.y.cmp(&self.p2.y)) {
      (Ordering::Equal, _) => Orientation::Vertical,
      (_, Ordering::Equal) => Orientation::Horizontal,
      (_, _) => Orientation::Diagonal
    }
  }

  fn points(&self) -> HashSet<Point> {
    let xstep = (self.p2.x - self.p1.x).signum();
    let ystep = (self.p2.y - self.p1.y).signum();

    let mut results = HashSet::new();
    let mut p = self.p1;

    while p != self.p2 {
      results.insert(p);

      p.x += xstep;
      p.y += ystep;
    }
    results.insert(self.p2);
    results
  }


  fn intersection_count(lines: &[Line]) -> usize {
    let mut results : HashSet<Point> = HashSet::new();

    let sets = lines
      .iter()
      .map(|line| line.points())
      .collect::<Vec<HashSet<Point>>>();

    for i in 0..(sets.len() - 1) {
      for j in (i + 1)..sets.len() {
        let points = sets[i].intersection(&sets[j]);
        results.extend(points);
      }
    }

    results.len()
  }
}


#[cfg(test)]
mod tests {

use super::*;

  #[test]
  fn part1_example() {
    let mut lines = input(SAMPLE).expect("shit");
    lines.retain(|line| line.orientation() != Orientation::Diagonal);

    assert_eq!(Line::intersection_count(&lines), 5);
  }

  #[test]
  fn part1_solution() {
    let mut lines = input(INPUT).expect("shit");
    lines.retain(|line| line.orientation() != Orientation::Diagonal);

    assert_eq!(Line::intersection_count(&lines), 5147);
  }

  #[test]
  fn part2_example() {
    let lines = input(SAMPLE).expect("shit");
    assert_eq!(Line::intersection_count(&lines), 12);
  }

  #[test]
  fn part2_solution() {
    let lines = input(INPUT).expect("shit");
    assert_eq!(Line::intersection_count(&lines), 16925);
  }
}
