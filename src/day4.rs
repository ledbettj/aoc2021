use std::str::FromStr;
use std::collections::HashSet;
use std::num::ParseIntError;

const INPUT : &'static str = include_str!("../inputs/day4.txt");
const SAMPLE : &'static str = include_str!("../inputs/day4.sample.txt");

#[derive(Debug, Clone, PartialEq)]
struct Board {
  numbers: Vec<Vec<usize>>
}

impl Board {
  pub fn is_won(&self, drawn: &HashSet<usize>) -> bool {
    self.has_won_row(&drawn) || self.has_won_col(&drawn)
  }

  fn has_won_row(&self, drawn: &HashSet<usize>) -> bool {
    self.numbers.iter().any(|row| row.iter().all(|n| drawn.contains(n)))
  }

  fn has_won_col(&self, drawn: &HashSet<usize>) -> bool {
    (0..self.numbers.len()).any(|index|{
      (0..self.numbers.len()).all(|n| drawn.contains(&self.numbers[n][index]))
    })
  }

  pub fn score(&self, drawn: &HashSet<usize>, last: usize) -> usize {
    let v : usize = self.numbers
      .iter()
      .flat_map(|row| row.iter().filter(|n| !drawn.contains(n)))
      .sum();

    v * last
  }

  pub fn load_problem(input: &str) -> Result<(Vec<usize>, Vec<Board>), ParseIntError> {
    let (first, rest) = input.split_once("\n").unwrap();
    let nums : Vec<usize> = first
      .split(",")
      .map(|item| item.parse::<usize>())
      .collect::<Result<Vec<_>, _>>()?;

    let boards : Vec<Board> = rest
      .trim()
      .split("\n\n")
      .map(|blob| blob.parse())
      .collect::<Result<Vec<_>, _>>()?;

    Ok((nums, boards))
  }

  pub fn calculate_first_winner<'a>(numbers: &[usize], boards: &'a [Board]) -> Option<usize> {
    let mut drawn  = HashSet::new();

    for i in 0..numbers.len() {
      drawn.insert(numbers[i]);

      if let Some(won) = boards.iter().find(|b| b.is_won(&drawn)) {
        return Some(won.score(&drawn, numbers[i]));
      }
    };
    None
  }

  pub fn calculate_last_winner<'a>(numbers: &[usize], boards: &'a [Board]) -> Option<usize> {
    let mut boards = boards.to_owned();
    let mut drawn = HashSet::new();

    for i in 0..numbers.len() {
      drawn.insert(numbers[i]);

      if boards.len() > 1 {
        boards.retain(|b| !b.is_won(&drawn));
      } else if boards[0].is_won(&drawn) {
          return Some(boards[0].score(&drawn, numbers[i]));
      }
    }
    None
  }
}

impl FromStr for Board {
  type Err = ParseIntError;

  fn from_str(s: &str) -> Result<Self, Self::Err> {
    let parts : Vec<Vec<usize>> = s
      .lines()
      .map(|line|{
        line
          .split_whitespace()
          .map(|n| n.parse::<usize>())
          .collect::<Result<Vec<_>, _>>()
      })
      .collect::<Result<Vec<_>,_>>()?;

    Ok(Board { numbers: parts } )
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn part1_example() {
    let input = SAMPLE;
    let (nums, boards) = Board::load_problem(&input).expect("shit");
    let score = Board::calculate_first_winner(&nums, &boards);

    assert_eq!(score, Some(4512));
  }

  #[test]
  fn part1_solution() {
    let input = INPUT;
    let (nums, boards) = Board::load_problem(&input).expect("shit");
    let score = Board::calculate_first_winner(&nums, &boards);

    assert_eq!(score, Some(89001));
  }

  #[test]
  fn part2_example() {
    let input = SAMPLE;
    let (nums, boards) = Board::load_problem(&input).expect("shit");
    let r = Board::calculate_last_winner(&nums, &boards);

    assert_eq!(r, Some(1924));
  }

  #[test]
  fn part2_solution() {
    let input = INPUT;
    let (nums, boards) = Board::load_problem(&input).expect("shit");
    let r = Board::calculate_last_winner(&nums, &boards);

    assert_eq!(r, Some(7296));
  }
}
