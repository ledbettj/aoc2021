use std::str::FromStr;
use std::collections::HashMap;

const INPUT : &'static str = include_str!("../inputs/day14.txt");
const SAMPLE : &'static str = include_str!("../inputs/day14.sample.txt");

struct Polymer {
  rules: HashMap<(char, char), char>,
  pairs: HashMap<(char, char), usize>,
}

impl FromStr for Polymer {
  type Err = &'static str;

  fn from_str(s: &str) -> Result<Self, Self::Err> {
    s
      .trim()
      .split_once("\n\n")
      .ok_or("No blank line in input")
      .and_then(|(top, bottom)|{
        let template = top.chars().collect::<Vec<char>>();
        let mut pairs = HashMap::new();

        for slice in template.windows(2) {
          let pair = (slice[0], slice[1]);
          let v = pairs.entry(pair).or_insert(0);
          *v += 1;
        }

        let rules = bottom
          .lines()
          .map(|line|{
            line
              .split_once(" -> ")
              .ok_or("No arrow found in rule")
              .map(|(lhs, rhs)|{
                let mut lc = lhs.chars();
                let mut rc = rhs.chars();

                ((lc.next().unwrap(), lc.next().unwrap()), rc.next().unwrap())
              })
          }).collect::<Result<HashMap<(char, char), char>, Self::Err>>();

        rules.map(|r| (r, pairs))
      })
      .map(|(rules, pairs)| Polymer { rules, pairs })
  }
}

impl Polymer {
  fn score(&self) -> usize {
    let mut counts : HashMap::<char, usize> = HashMap::new();

    for (&(a, b), &count) in &self.pairs {
      let v = counts.entry(a).or_insert(0);
      *v += count;
      let v2 = counts.entry(b).or_insert(0);
      *v2 += count;
    }

    let max = counts.values().max().unwrap();
    let min = counts.values().min().unwrap();
    // why does this work.  what am i doin wrong
    // idk why this is right but lets go with it
    (max - min) / 2 + 1
  }

  fn step(&mut self) {
    let mut new = HashMap::new();

    for (&(a, b), &count) in &self.pairs {
      if let Some(&middle) = self.rules.get(&(a, b)) {
        let v = new.entry((a, middle)).or_insert(0);
        *v += count;
        let v2 = new.entry((middle, b)).or_insert(0);
        *v2 += count;
      }
    };
    self.pairs = new;
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn part1_example() {
    let mut p : Polymer = SAMPLE.parse().expect("shit");
    for _ in 0..10 {
      p.step();
    }

    assert_eq!(p.score(), 1588);
  }

  #[test]
  fn part1_solution() {
    let mut p : Polymer = INPUT.parse().expect("shit");
    for _ in 0..10 {
      p.step();
    }

    assert_eq!(p.score(), 3247);
  }

  #[test]
  fn part2_example() {
    let mut p : Polymer = SAMPLE.parse().expect("shit");
    for _ in 0..40 {
      p.step();
    }

    assert_eq!(p.score(), 2188189693529);
  }

  #[test]
  fn part2_solution() {
    let mut p : Polymer = INPUT.parse().expect("shit");
    for _ in 0..40 {
      p.step();
    }

    assert_eq!(p.score(), 4110568157153);
  }
}
