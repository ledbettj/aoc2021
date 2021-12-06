use std::{str::FromStr, num::ParseIntError};

const INPUT : &'static str = include_str!("../inputs/day6.txt");
const SAMPLE : &'static str = "3,4,3,1,2";

#[derive(Debug)]
struct School {
  fish: [usize; 9]
}

impl School {
  pub fn tick(&mut self) {
    let zeroes = self.fish[0];

    (1..9).for_each(|index| self.fish[index - 1] = self.fish[index]);

    self.fish[6] += zeroes;
    self.fish[8] = zeroes;
  }

  pub fn run(&mut self, generations: usize) -> &Self {
    (0..generations).for_each(|_| self.tick());
    self
  }

  pub fn len(&self) -> usize {
    self.fish.iter().sum()
  }
}

impl FromStr for School {
  type Err = ParseIntError;

  fn from_str(s: &str) -> Result<Self, Self::Err> {
    s
      .trim()
      .split(",")
      .map(|ch| ch.parse::<usize>())
      .try_fold([0; 9], |mut counts, digit| {
        digit
          .map(|index| {
            counts[index] += 1;
            counts
          })
      })
      .map(|fish| School { fish })
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn part1_example() {
    let mut school : School = SAMPLE.parse().expect("shit");
    assert_eq!(school.run(18).len(), 26);
  }

  #[test]
  fn part1_solution() {
    let mut school : School = INPUT.parse().expect("shit");
    assert_eq!(school.run(80).len(), 388739);
  }

  #[test]
  fn part2_example() {
    let mut school : School = SAMPLE.parse().expect("shit");
    assert_eq!(school.run(256).len(), 26984457539);
  }

  #[test]
  fn part2_solution() {
    let mut school : School = INPUT.parse().expect("shit");
    assert_eq!(school.run(256).len(), 1741362314973);
  }
}
