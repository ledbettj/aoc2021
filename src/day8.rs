use std::{str::FromStr, error::Error, io, collections::{HashMap, HashSet}};

const INPUT: &'static str = include_str!("../inputs/day8.txt");
const SAMPLE: &'static str = include_str!("../inputs/day8.sample.txt");

struct Sample {
  samples: HashSet<String>,
  outputs: Vec<String>,
}

impl FromStr for Sample {
  type Err = Box<dyn Error>;

  fn from_str(s: &str) -> Result<Self, Self::Err> {
    let (samples, outputs) = s
      .trim()
      .split_once(" | ")
      .ok_or(io::Error::new(io::ErrorKind::UnexpectedEof, "Missing delim"))?;

    let fixup = |word: &str| {
      let mut v : Vec<char> = word.chars().collect();
      v.sort();
      v.iter().collect::<String>()
    };

    let samples = samples
      .split_whitespace()
      .map(fixup)
      .collect();
    let outputs = outputs
      .split_whitespace()
      .map(fixup)
      .collect();

    Ok(Sample { samples, outputs })
  }
}

impl Sample {
  fn count_p1(&self) -> usize {
    self.outputs
      .iter()
      .filter(|word| (word.len() >= 2 && word.len() <= 4) || word.len() == 7)
      .count()
  }

  fn contains_other(s1: &str, other: &str) -> bool {
    let one = s1.chars().collect::<HashSet<char>>();
    let two = other.chars().collect::<HashSet<char>>();

    two.iter().all(|item| one.contains(item))
  }

  fn contains_other_count(s1: &str, other: &str, count: usize) -> bool {
    let one = s1.chars().collect::<HashSet<char>>();
    let two = other.chars().collect::<HashSet<char>>();

    two.iter().filter(|item| one.contains(item)).count() == count
  }


  fn decode(&self) -> usize {
    let mut lookup : HashMap<String, u8> = HashMap::new();
    let mut rev: HashMap<u8, String> = HashMap::new();

    for s in &self.samples {
      match s.len() {
        2 => { lookup.insert(s.clone(), 1); rev.insert(1, s.clone()); },
        3 => { lookup.insert(s.clone(), 7); rev.insert(7, s.clone()); },
        4 => { lookup.insert(s.clone(), 4); rev.insert(4, s.clone());  },
        5 => {  /* nop */ }, // [2, 3, 5]
        6 => {  /* nop */ }, // [0, 6, 9]
        7 => { lookup.insert(s.clone(), 8); },
        d => panic!("Impossible input: {}", d)
      };
    }

    let one = rev.get(&1).unwrap();
    let seven = rev.get(&7).unwrap();
    let four = rev.get(&4).unwrap();

    self.samples
      .iter()
      .filter(|s| s.len() == 5)
      .for_each(|s| {
        // if it contains 1 it's 3
        if Sample::contains_other(s, one) {
          lookup.insert(s.clone(), 3);
        }
        // if it contains 3 out of 4 parts of 4 it's 5
        else if Sample::contains_other_count(s, four, 3) {
          lookup.insert(s.clone(), 5);
        } else {
          lookup.insert(s.clone(), 2);
        }
      });

    self.samples
      .iter()
      .filter(|s| s.len() == 6)
      .for_each(|s| {
        // if it DOESNT contains 7 it's 6
        if !Sample::contains_other(s, seven) {
          lookup.insert(s.clone(), 6);
        }
        // if it contains 4 it's 9
        else if Sample::contains_other(s, four) {
          lookup.insert(s.clone(), 9);
        }
        else {
          lookup.insert(s.clone(), 0);
        }
      });


    self.outputs
      .iter()
      .map(|s| *lookup.get(s).unwrap())
      .fold(0usize, |accum, digit| {
        accum * 10 + (digit as usize)
      })
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn part1_example() {
    let s = SAMPLE.lines().map(|line| line.parse()).collect::<Result<Vec<Sample>, _>>().expect("shit");

    let ans : usize = s.iter()
      .map(|sample| sample.count_p1())
      .sum();

    assert_eq!(ans, 26);
  }

  #[test]
  fn part1_solution() {
    let s = INPUT.lines().map(|line| line.parse()).collect::<Result<Vec<Sample>, _>>().expect("shit");
    let ans : usize = s.iter()
      .map(|sample| sample.count_p1())
      .sum();

    assert_eq!(ans, 470);
  }

  #[test]
  fn part2_example() {
    let s = SAMPLE.lines().map(|line| line.parse()).collect::<Result<Vec<Sample>, _>>().expect("shit");

    let ans : usize = s.iter().map(|sample| sample.decode()).sum();
    assert_eq!(ans, 61229);
  }

  #[test]
  fn part2_solution() {
    let s = INPUT.lines().map(|line| line.parse()).collect::<Result<Vec<Sample>, _>>().expect("shit");

    let ans : usize = s.iter().map(|sample| sample.decode()).sum();
    assert_eq!(ans, 989396);
  }
}
