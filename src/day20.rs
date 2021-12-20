use std::{collections::HashMap, str::FromStr, fmt::Display};

const INPUT : &'static str = include_str!("../inputs/day20.txt");
const SAMPLE : &'static str = include_str!("../inputs/day20.sample.txt");
const SAMPLE2 : &'static str = include_str!("../inputs/day20.sample2.txt");

type Point = (isize, isize);

struct Image {
  rules: Vec<char>,
  pixels: HashMap<Point, char>
}

impl FromStr for Image {
  type Err = &'static str;

  fn from_str(s: &str) -> Result<Self, Self::Err> {
    s
      .trim()
      .split_once("\n\n")
      .ok_or("No blank line in input")
      .map(|(top, bottom)|{
        let rules : Vec<char> = top.chars().collect();
        let pixels = bottom
          .lines()
          .enumerate()
          .flat_map(|(y, line)|{
            line
              .chars()
              .enumerate()
//              .filter(|(_, ch)| *ch == '#')
              .map(move |(x, ch)| ((x as isize, y as isize), ch))
          }).collect::<HashMap<Point, char>>();

        Self { pixels, rules }
      })
  }
}

impl Image {
  fn min_x(&self) -> isize {
    self.pixels.keys().map(|&(x, _)| x).min().unwrap_or(0) - 2
  }

  fn max_x(&self) -> isize {
    self.pixels.keys().map(|&(x, _)| x).max().unwrap_or(0) + 2
  }

  fn min_y(&self) -> isize {
    self.pixels.keys().map(|&(_, y)| y).min().unwrap_or(0) - 2
  }

  fn max_y(&self) -> isize {
    self.pixels.keys().map(|&(_, y)| y).max().unwrap_or(0) + 2
  }

  fn enhance(&mut self, default: char) {

    let s = &self;

    let pixels = (self.min_y()..=self.max_y())
      .into_iter()
      .flat_map(|y|{
        (self.min_x()..=self.max_x()).map(move |x|{
          ((x, y), s.enhance_pixel(&(x, y), default))
        })
      })
//      .filter(|(_, ch)| *ch == '#')
      .collect();

    self.pixels = pixels;
  }

  fn enhance_pixel(&self, pos: &Point, default: char) -> char {
    let s = self.grid_at(pos, default);
    self.lookup_for(&s)
  }

  fn lookup_for(&self, s: &str) -> char {
    let index = s
      .chars()
      .fold(0, |mut value, ch|{
        value <<= 1;
        if ch == '#' {
          value |= 1;
        }
        value
      });

    self.rules[index]
  }

  fn grid_at(&self, pos: &Point, default: char) -> String {
    ((pos.1 - 1)..=(pos.1 + 1))
      .into_iter()
      .flat_map(|y|{
        ((pos.0 - 1)..=(pos.0 + 1)).map(move |x|{
          *self
            .pixels
            .get(&(x, y))
            .unwrap_or(&default)
        })
      })
      .collect()
  }

  fn lit_count(&self) -> usize {
    self.pixels.values().filter(|&v| *v == '#').count()
  }
}

impl Display for Image {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    for y in self.min_y()..=self.max_y() {
      for x in self.min_x()..=self.max_x() {
        write!(f, "{}", self.pixels.get(&(x, y)).unwrap_or(&'.'))?;
      }
      write!(f, "\n")?;
    }
    Ok(())
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn part1_example() {
    let mut i : Image = SAMPLE.parse().expect("shit");
    println!("lit block is {} dim block is {}", i.lookup_for("#########"), i.lookup_for("........."));

    i.enhance('.');
    i.enhance('.');

    assert_eq!(i.lit_count(), 35);
  }

  #[test]
  fn part1_example2() {
    let mut i : Image = SAMPLE2.parse().expect("shit");
    i.enhance('.');
    i.enhance('#');
    assert_eq!(i.lit_count(), 5326);
  }


  #[test]
  fn part1_solution() {
    let mut i : Image = INPUT.parse().expect("shit");
    i.enhance('.');
    i.enhance('#');
    assert_eq!(i.lit_count(), 5498);
  }

  #[test]
  fn part2_example() {
    let defaults = &['.', '.'];

    let mut i : Image = SAMPLE.parse().expect("shit");
    for x in 0..50 {
      i.enhance(defaults[x % 2])
    }
    assert_eq!(i.lit_count(), 3351);
  }

  #[test]
  fn part2_solution() {
    let defaults = &['.', '#'];

    let mut i : Image = INPUT.parse().expect("shit");
    for x in 0..50 {
      i.enhance(defaults[x % 2])
    }
    assert_eq!(i.lit_count(), 16014);
  }
}
