use std::fmt::Display;
use std::str::FromStr;
use std::collections::HashMap;

const INPUT : &'static str = include_str!("../inputs/day14.txt");
const SAMPLE : &'static str = include_str!("../inputs/day14.sample.txt");

struct Polymer {
  template: Vec<char>,
  rules: HashMap<(char, char), char>
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

        rules.map(|r| (r, template))
      })
      .map(|(rules, template)| Polymer { rules, template })
  }
}

impl Display for Polymer {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "{}", self.template.iter().collect::<String>())
  }
}

impl Polymer {
  fn score(&self) -> usize {
    let mut counts : HashMap::<char, usize> = HashMap::new();

    for &ch in &self.template {
      let v = counts.entry(ch).or_insert(0);
      *v += 1;
    }

    let max = counts.values().max().unwrap();
    let min = counts.values().min().unwrap();

    max - min
  }

  fn step(&mut self) {
    let mut v = Vec::with_capacity(self.template.len() * 3);
    let mut last = 'z';
    
    for slice in self.template.windows(2) {
      let chars = (slice[0], slice[1]);
      v.push(chars.0);
      if let Some(ch) = self.rules.get(&chars) {
        v.push(*ch);
      }
      last = chars.1;
    };
    v.push(last);

    self.template = v;
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

    assert_eq!(p.template.len(), 3073);
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
    // let mut p : Polymer = SAMPLE.parse().expect("shit");
    // for _ in 0..40 {
    //   p.step();
    // }

    // assert_eq!(p.score(), 2188189693529);
  }

  #[test]
  fn part2_solution() {

  }
}
