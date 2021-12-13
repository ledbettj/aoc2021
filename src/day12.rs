use std::collections::{HashMap,HashSet};
use std::str::FromStr;

const INPUT : &'static str = include_str!("../inputs/day12.txt");
const SAMPLE : &'static str = include_str!("../inputs/day12.sample.txt");

struct Caves {
  caves: HashMap<String, HashSet<String>>
}

impl FromStr for Caves {
  type Err = &'static str;

  fn from_str(s: &str) -> Result<Self, Self::Err> {
    s
      .trim()
      .lines()
      .map(|line|{
        line
          .split_once("-")
          .ok_or("No dash in line")
          .map(|(lhs, rhs)| (lhs.to_owned(), rhs.to_owned()))
      })
      .try_fold(HashMap::new(), |mut hash, parts|{
        parts.map(|(lhs, rhs)|{

          hash
            .entry(lhs.clone())
            .or_insert(HashSet::new())
            .insert(rhs.clone());

          hash
            .entry(rhs)
            .or_insert(HashSet::new())
            .insert(lhs);

          hash
        })
      })
      .map(|hash| Caves { caves: hash })
  }
}

impl Caves {
  pub fn total_paths(&self) -> usize {
    self.paths_from("start", "end", HashSet::new())
  }

  pub fn paths_from(&self, from: &str, to: &str, mut forbidden: HashSet<String>) -> usize {
    forbidden.insert(from.to_owned());

    if from == to {
      return 1;
    }

    self
      .neighbors(from)
      .iter()
      .filter(|&neighbor| !Caves::is_small(neighbor) || !forbidden.contains(neighbor))
      .map(|neighbor| self.paths_from(neighbor, to, forbidden.clone()))
      .sum()
  }


  pub fn total_paths_2(&self) -> usize {
    self.paths_from_2("start", "end", HashMap::new())
  }

  pub fn neighbors(&self, node: &str) -> &HashSet<String> {
    self.caves.get(node).unwrap()
  }

  pub fn paths_from_2(&self, from: &str, to: &str, mut forbidden: HashMap<String, usize>) -> usize {
    let count = forbidden
      .entry(from.to_owned())
      .or_insert(0);

    *count += 1;

    if from == to {
      return 1;
    }

    self
      .neighbors(from)
      .iter()
      .filter(|&neighbor|{
        !Caves::is_small(neighbor) || // big caves A ok
          !forbidden.contains_key(neighbor) || // never before seen cave sounds great
          (forbidden
           .iter()
           .filter(|(k, _)| Caves::is_small(k)) // all small caves
           .all(|(_, &v)| v == 1) // previously visited only once
           && neighbor != "start" // cant go back to start u dipshit
          )
      })
      .map(|neighbor| self.paths_from_2(neighbor, to, forbidden.clone()))
      .sum()
  }

  fn is_small(cave: &str) -> bool {
    cave.to_lowercase() == cave
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn part1_example() {
    let c : Caves = SAMPLE.parse().expect("shit");
    assert_eq!(c.total_paths(), 10);
  }

  #[test]
  fn part1_solution() {
    let c : Caves = INPUT.parse().expect("shit");
    assert_eq!(c.total_paths(), 5576);
  }

  #[test]
  fn part2_example() {
    let c : Caves = SAMPLE.parse().expect("shit");
    assert_eq!(c.total_paths_2(), 36);

  }

  #[test]
  fn part2_solution() {
    let c : Caves = INPUT.parse().expect("shit");
    assert_eq!(c.total_paths_2(), 152837);
  }
}
