use std::{collections::{HashSet, VecDeque}, str::FromStr, fmt::Display};


const INPUT : &'static str = include_str!("../inputs/day13.txt");
const SAMPLE : &'static str = include_str!("../inputs/day13.sample.txt");

type Point = (isize, isize);

#[derive(Clone,Copy,Debug)]
enum Fold {
  Up(isize),
  Left(isize),
}

struct Paper {
  points: HashSet<Point>,
  folds:  VecDeque<Fold>,
}

impl FromStr for Fold {
  type Err = &'static str;

  fn from_str(s: &str) -> Result<Self, Self::Err> {
    s
      .rsplit_once(" ")
      .ok_or("No space in input")
      .map(|(_, rhs)| rhs)
      .and_then(|rhs| rhs.split_once("=").ok_or("No equal sign in input"))
      .and_then(|(xy, val)|{
        let v = val.parse::<isize>().map_err(|_| "Failed to parse value")?;
        match xy {
          "x" => Ok(Fold::Left(v)),
          "y" => Ok(Fold::Up(v)),
          _   => Err("Invalid fold type")
        }
      })
  }
}

impl FromStr for Paper {
  type Err = &'static str;

  fn from_str(s: &str) -> Result<Self, Self::Err> {
    s
      .trim()
      .split_once("\n\n")
      .ok_or("No blank line in input")
      .and_then(|(top, bottom)|{
        let points = top
          .lines()
          .map(|line|{
            line
              .split_once(",")
              .ok_or("No comma in point")
              .and_then(|parts|{
                let x = parts.0.parse::<isize>().map_err(|_| "Failed to parse x")?;
                let y = parts.1.parse::<isize>().map_err(|_| "Failed to parse y")?;
                Ok((x, y))
              })
          }).collect::<Result<HashSet<Point>, Self::Err>>()?;
        let folds = bottom
          .lines()
          .map(|line| line.parse())
          .collect::<Result<VecDeque<Fold>, Self::Err>>()?;

        Ok(Paper { points, folds })
      })
  }
}

impl Display for Paper {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    let (minx, miny) = (0, 0);
    let &maxx = self.points.iter().map(|(x, _)| x).max().unwrap();
    let &maxy = self.points.iter().map(|(_, y)| y).max().unwrap();

    for y in miny..=maxy {
      for x in minx..=maxx {
        if self.points.contains(&(x, y)) {
          write!(f, "#")?;
        } else {
          write!(f, " ")?;
        }
      }
      write!(f, "\n")?;
    }
    Ok(())
  }
}

impl Paper {
  fn fold_all(&mut self) {
    while !self.folds.is_empty() {
      self.fold_once();
    }
  }

  fn fold_once(&mut self) {
    if let Some(f) = self.folds.pop_front() {
      let mut to_remove : Vec<Point> = vec![];

      for p in &self.points {
        match f {
          Fold::Left(x) if p.0 > x => { to_remove.push(*p); },
          Fold::Up(y)   if p.1 > y => { to_remove.push(*p); },
          _ => { /* nop */ }
        };
      };

      to_remove
        .into_iter()
        .map(|(px, py)|{
          let new_point = match f {
            Fold::Left(x) => (x - (px - x), py),
            Fold::Up(y)   => (px, y - (py - y)),
          };
          ((px, py), new_point)
        })
        .for_each(|(old_point, new_point)|{
          self.points.remove(&old_point);
          self.points.insert(new_point);
        });

    }
  }

  fn len(&self) -> usize {
    self.points.len()
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn part1_example() {
    let mut paper : Paper = SAMPLE.parse().expect("shit");
    paper.fold_once();
    assert_eq!(paper.len(), 17);
  }

  #[test]
  fn part1_solution() {
    let mut paper : Paper = INPUT.parse().expect("shit");
    paper.fold_once();
    assert_eq!(paper.len(), 827);

  }

  #[test]
  fn part2_example() {
    let mut paper : Paper = SAMPLE.parse().expect("shit");
    paper.fold_all();
    println!("{}", paper);
  }

  #[test]
  fn part2_solution() {
    let mut paper : Paper = INPUT.parse().expect("shit");
    paper.fold_all();
    println!("{}", paper);
  }
}
