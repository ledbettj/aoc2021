use std::{str::FromStr, fmt::Display};
use itertools::Itertools;

const INPUT : &'static str = include_str!("../inputs/day18.txt");

#[derive(Debug, Clone, Eq, PartialEq)]
enum Snailfish {
  Number(u32),
  Pair(Vec<Snailfish>)
}

#[derive(Debug, Eq, PartialEq)]
enum Explode {
  Nope,
  Yup(Option<u32>, Option<u32>),
}

impl FromStr for Snailfish {
  type Err = &'static str;

  fn from_str(s: &str) -> Result<Self, Self::Err> {
    let mut stack = vec![];

    for ch in s.trim().chars() {
      match ch {
        '[' | ',' => { /* nada */ },
        ']' => {
          let r = stack.pop().ok_or("Parse error")?;
          let l = stack.pop().ok_or("Parse error")?;
          stack.push(Snailfish::Pair(vec![l, r]));
        },
        _ => {
          stack.push(Snailfish::Number(ch.to_digit(10).ok_or("Invalid digit character")?));
        }
      }
    }

    stack.pop().ok_or("Parse error")
  }
}

impl Snailfish {
  fn add(self, other: Snailfish) -> Snailfish {
    let mut result = Snailfish::Pair(vec![self, other]);
    result.reduce();
    result
  }

  fn magnitude(&self) -> u32 {
    match self {
      Snailfish::Number(v) => *v,
      Snailfish::Pair(children) => 3 * children[0].magnitude() + 2 * children[1].magnitude(),
    }
  }

  // carry the digit from an explosion to the right.
  // if we absorb the digit, returns None, otherwise Some(digit)
  fn carry_right(&mut self, value: u32) -> Option<u32> {
    match self {
      Snailfish::Number(v) => { *v += value; None },
      Snailfish::Pair(children) => {
        if children.iter_mut().any(|child| child.carry_right(value).is_none()) {
          None
        } else {
          Some(value)
        }
      }
    }
  }

  // carry the digit from an explosion to the left. (reverse child order)
  // if we absorb the digit, returns None, otherwise Some(digit)
  fn carry_left(&mut self, value: u32) -> Option<u32> {
    match self {
      Snailfish::Number(v) => { *v += value; None },
      Snailfish::Pair(children) => {
        if children.iter_mut().rev().any(|child| child.carry_left(value).is_none()) {
          None
        } else {
          Some(value)
        }
      }
    }
  }

  fn explode(&mut self, depth: usize) -> Explode {
    match self {
      Snailfish::Pair(children) => {
        // check left child
        if let Explode::Yup(left, mut right) = children[0].explode(depth + 1) {
          // attempt to absorb the right hand carry
          right = right.and_then(|val| children[1].carry_right(val));
          return Explode::Yup(left, right);
        };

        // check self
        if depth >= 4 {
          let values = children.iter().cloned().map(|sn| match sn {
            Snailfish::Number(v) => Some(v),
            _ => None,
          }).collect::<Vec<Option<u32>>>();

          *self = Snailfish::Number(0);
          return Explode::Yup(values[0], values[1])
        }

        // check right child
        if let Explode::Yup(mut left, right) = children[1].explode(depth + 1) {
          // attempt to absorb the left hand carry
          left = left.and_then(|val| children[0].carry_left(val));
          return Explode::Yup(left, right);
        }

        return Explode::Nope;
      },
      _ => return Explode::Nope
    };
  }

  fn reduce(&mut self) {
    let mut done = false;

    while !done {
      done = true;

      while self.explode(0) != Explode::Nope {
        done = false;
      }

      if self.split() {
        done = false;
      }
    }
  }

  fn split(&mut self) -> bool {
    match self {
      Snailfish::Number(v) if *v >= 10 => {
        let lh = *v / 2;
        let rh = *v - lh;
        *self = Snailfish::Pair(vec![Snailfish::Number(lh), Snailfish::Number(rh)]);
        true
      },
      Snailfish::Pair(children) => children.iter_mut().any(|c| c.split()),
      _ => false,
    }
  }
}

impl Display for Snailfish {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    match self {
      Snailfish::Number(v) => write!(f, "{}", v),
      Snailfish::Pair(children) => write!(f, "[{},{}]", children[0], children[1])
    }
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_parse() {
    let s : Snailfish = "[1,2]".parse().expect("shit");
    assert_eq!(s, Snailfish::Pair(vec![Snailfish::Number(1), Snailfish::Number(2)]));
    let s : Snailfish = "[[1,2],3]".parse().expect("shit");
    assert_eq!(s, Snailfish::Pair(vec![
      Snailfish::Pair(vec![Snailfish::Number(1), Snailfish::Number(2)]),
      Snailfish::Number(3)])
    );
  }

  #[test]
  fn test_explode() {
    let mut s : Snailfish  = "[[[[[9,8],1],2],3],4]".parse().unwrap();
    s.explode(0);
    assert_eq!(s, "[[[[0,9],2],3],4]".parse().unwrap());

    s = "[7,[6,[5,[4,[3,2]]]]]".parse().unwrap();
    s.explode(0);
    assert_eq!(s, "[7,[6,[5,[7,0]]]]".parse().unwrap());

    s = "[[6,[5,[4,[3,2]]]],1]".parse().unwrap();
    s.explode(0);
    assert_eq!(s, "[[6,[5,[7,0]]],3]".parse().unwrap());

    s = "[[3,[2,[1,[7,3]]]],[6,[5,[4,[3,2]]]]]".parse().unwrap();
    s.explode(0);
    assert_eq!(s, "[[3,[2,[8,0]]],[9,[5,[4,[3,2]]]]]".parse().unwrap());

    s.explode(0);
    assert_eq!(s, "[[3,[2,[8,0]]],[9,[5,[7,0]]]]".parse().unwrap());
  }
  #[test]
  fn jake() {
    let l : Snailfish = "[[[0,[5,8]],[[1,7],[9,6]]],[[4,[1,2]],[[1,4],2]]]".parse().unwrap();
    let r : Snailfish = "[[[5,[2,8]],4],[5,[[9,9],0]]]".parse().unwrap();
    let x : Snailfish = "[6,[[[6,2],[5,6]],[[7,6],[4,7]]]]".parse().unwrap();
    println!("{}", l.add(r).add(x));
  }
  #[test]
  fn part1_example() {
    let input = "[[[0,[5,8]],[[1,7],[9,6]]],[[4,[1,2]],[[1,4],2]]]
[[[5,[2,8]],4],[5,[[9,9],0]]]
[6,[[[6,2],[5,6]],[[7,6],[4,7]]]]
[[[6,[0,7]],[0,9]],[4,[9,[9,0]]]]
[[[7,[6,4]],[3,[1,3]]],[[[5,5],1],9]]
[[6,[[7,3],[3,2]]],[[[3,8],[5,7]],4]]
[[[[5,4],[7,7]],8],[[8,3],8]]
[[9,3],[[9,9],[6,[4,9]]]]
[[2,[[7,7],7]],[[5,8],[[9,3],[0,2]]]]
[[[[5,2],5],[8,[3,7]]],[[5,[7,5]],[4,4]]]";

    let fish : Vec<Snailfish>  = input
      .lines()
      .map(|line| line.parse::<Snailfish>()).collect::<Result<Vec<Snailfish>, _>>().expect("shit");

    let result = fish
      .into_iter()
      .reduce(move |left, right| left.add(right))
      .unwrap();

    assert_eq!(result.magnitude(), 4140);

    let m : Snailfish = "[[[[8,7],[7,7]],[[8,6],[7,7]]],[[[0,7],[6,6]],[8,7]]]".parse().expect("shit");
    assert_eq!(m.magnitude(), 3488);
  }

  #[test]
  fn part1_solution() {
    let fish : Vec<Snailfish>  = INPUT
      .lines()
      .map(|line| line.parse::<Snailfish>()).collect::<Result<Vec<Snailfish>, _>>().expect("shit");

    let result = fish
      .into_iter()
      .reduce(move |left, right| left.add(right))
      .unwrap();

    assert_eq!(result.magnitude(), 4457);
  }

  #[test]
  fn part2_solution() {
    let fish : Vec<Snailfish>  = INPUT
      .lines()
      .map(|line| line.parse::<Snailfish>()).collect::<Result<Vec<Snailfish>, _>>().expect("shit");

    let max = fish
      .into_iter()
      .permutations(2)
      .map(|p| p[0].clone().add(p[1].clone()).magnitude())
      .max();
    assert_eq!(max, Some(4784));
  }
}
