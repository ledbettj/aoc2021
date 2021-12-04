use std::str::FromStr;
use std::collections::HashSet;
use std::num::ParseIntError;

const INPUT : &'static str = include_str!("../inputs/day4.txt");
const SAMPLE : &'static str = include_str!("../inputs/day4.sample.txt");

#[derive(Debug, Clone)]
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

  fn score(&self, drawn: &HashSet<usize>, last: usize) -> usize {
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

    let mut boards : Vec<Board> = vec![];

    // fuck you iterators
    let lines = rest.lines().filter(|line| !line.trim().is_empty()).collect::<Vec<&str>>();
    //println!("lines is {:?}", lines);

    for chunk in lines.chunks_exact(5) {
      let blob = chunk.join("\n");
      boards.push(blob.parse()?);
    }

    Ok((nums, boards))
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

    for i in (0..nums.len()) {
      let drawn : HashSet<usize> = nums.iter().take(i).cloned().collect();
      if let Some(won) = boards.iter().find(|b| b.is_won(&drawn)) {
        println!("board {:?} won after {:?} draws score = {}", boards, i, won.score(&drawn, nums[i - 1]));
        break;
      }
    }
  }

  #[test]
  fn part1_solution() {
    let input = INPUT;
    let (nums, boards) = Board::load_problem(&input).expect("shit");

    for i in (0..nums.len()) {
      let drawn : HashSet<usize> = nums.iter().take(i).cloned().collect();
      if let Some(won) = boards.iter().find(|b| b.is_won(&drawn)) {
        println!("board {:?} won after {:?} draws score = {}", boards, i, won.score(&drawn, nums[i - 1]));
        break;
      }
    }
  }

  #[test]
  fn part2_example() {

  }

  #[test]
  fn part2_solution() {
    let input = INPUT;
    let (nums, mut boards) = Board::load_problem(&input).expect("shit");

    for i in (0..nums.len()) {
      let drawn : HashSet<usize> = nums.iter().take(i).cloned().collect();

      if boards.len() == 1 {
        if boards[0].is_won(&drawn) {
          println!("board {:?} finally won after {:?} draws score = {}", boards[0], i, boards[0].score(&drawn, nums[i - 1]));
          break;
        }
      } else {
        boards = boards.iter().filter(|b| !b.is_won(&drawn)).cloned().collect();
      }
    }
  }
}
