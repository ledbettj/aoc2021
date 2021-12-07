use std::collections::HashMap;
use std::num::ParseIntError;
use std::str::FromStr;

const INPUT: &'static str = include_str!("../inputs/day7.txt");
const SAMPLE: &'static str = "16,1,2,0,4,2,7,1,2,14";

struct Crabs {
  crabs: HashMap<isize, isize>,
  median: isize,
}

fn calc_median(values: &[isize]) -> isize {
  let len = values.len();
  if len & 1 == 1 {
    (values[len / 2] + values[len / 2 + 1]) / 2
  } else {
    values[len / 2]
  }
}

impl FromStr for Crabs {
  type Err = ParseIntError;

  fn from_str(s: &str) -> Result<Self, Self::Err> {
    let mut values: Vec<isize> = s
      .trim()
      .split(",")
      .map(|n| n.parse())
      .collect::<Result<Vec<_>, _>>()?;

    values.sort();
    // p1 lowest cost is the median position
    let median = calc_median(&values);

    let crabs = values.iter().fold(HashMap::new(), |mut hash, &pos| {
      hash.entry(pos).and_modify(|v| *v += 1).or_insert(1);
      hash
    });

    Ok(Crabs { crabs, median })
  }
}

impl Crabs {
  pub fn lowest_cost<F>(&self, cost_func: F) -> Option<isize>
  where
    F: Fn(isize, isize) -> isize,
  {
    let min: isize = *self.crabs.keys().min()?;
    let max: isize = *self.crabs.keys().max()?;

    (min..=max)
      .map(|target| self.cost_for_target(target, &cost_func))
      .min()
  }

  fn cost_for_target<F>(&self, target: isize, cost_func: F) -> isize
  where
    F: Fn(isize, isize) -> isize,
  {
    self
      .crabs
      .iter()
      .map(|(&pos, &count)| cost_func(pos, target) * count)
      .sum()
  }
}

fn cost_p2(pos: isize, target: isize) -> isize {
  let distance = (target - pos).abs();
  (distance * (distance + 1)) / 2
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn part1_example() {
    let c: Crabs = SAMPLE.parse().expect("shit");
    assert_eq!(c.cost_for_target(c.median, |p, t| (t - p).abs()), 37);
  }

  #[test]
  fn part1_solution() {
    let c: Crabs = INPUT.parse().expect("shit");
    assert_eq!(c.cost_for_target(c.median, |p, t| (t - p).abs()), 343441);
  }

  #[test]
  fn part2_example() {
    let c: Crabs = SAMPLE.parse().expect("shit");
    assert_eq!(c.lowest_cost(cost_p2), Some(168));
  }

  #[test]
  fn part2_solution() {
    let c: Crabs = INPUT.parse().expect("shit");
    assert_eq!(c.lowest_cost(cost_p2), Some(98925151));
  }
}
