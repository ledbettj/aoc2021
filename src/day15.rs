use std::{collections::{HashMap, HashSet}, str::FromStr};

const INPUT : &'static str = include_str!("../inputs/day15.txt");
const SAMPLE : &'static str = include_str!("../inputs/day15.sample.txt");

type Point = (isize, isize);

struct Grid {
  points: HashMap<Point, isize>
}

impl FromStr for Grid {
  type Err = &'static str;

  fn from_str(s: &str) -> Result<Self, Self::Err> {
    s
      .trim()
      .lines()
      .enumerate()
      .flat_map(|(y, row)|{
        row
          .chars()
          .enumerate()
          .map(move |(x, ch)|{
            ch
              .to_digit(10)
              .ok_or("Failed to parse digit")
              .map(|value| ((x as isize, y as isize), value as isize))
          })
      })
      .collect::<Result<HashMap<Point, isize>, Self::Err>>()
      .map(|points| Grid { points })
  }
}

impl Grid {
  pub fn explode(&mut self) {
    let (maxx, maxy) = *self.points.keys().max().unwrap();
    let width  = maxx + 1;

    let risk = |value: isize, offset: isize| {
      if value + offset < 10 {
        value + offset
      } else {
        // zero isn't used so if we wrap around we start at 1
        ((value + offset) % 10) + 1
      }
    };

    // this is ugly, first duplicate the block horizontally 5 times
    for x in 0..=maxx {
      for y in 0..=maxy {
        for i in 1..5 {
          let v = self.points[&(x, y)];
          self.points.insert((x + width * i, y), risk(v, i));
        }
      }
    }

    let (maxx, maxy) = *self.points.keys().max().unwrap();
    let height = maxy + 1;

    // then duplicate vertically 5 times
    for x in 0..=maxx {
      for y in 0..=maxy {
        for i in 1..5 {
          let v = self.points[&(x, y)];
          self.points.insert((x, y + height * i), risk(v, i));
        }
      }
    }
  }

  pub fn lowest_risk_path(&self) -> isize {
    let mut costs : HashMap<Point, isize>  = HashMap::new();
    let dest = *self.points.keys().max().unwrap();
    let mut cur = (0, 0);
    let mut visited : HashSet<Point> = HashSet::new();
    let mut to_visit : Vec<Point> = vec![];

    costs.insert(cur, 0);
    visited.insert(cur);

    while cur != dest {
      self
        .neighbors(cur)
        .into_iter()
        .filter(|p| !visited.contains(p))
        .for_each(|p|{
          let total = costs[&cur] + self.points.get(&p).unwrap();
          if !costs.contains_key(&p) {
            costs.insert(p, total);
            to_visit.push(p);
          };
        });

      cur = *to_visit.iter().min_by_key(|p| costs[&p]).unwrap();
      to_visit.retain(|&node| node != cur);

      visited.insert(cur);
    }

    costs[&dest]
  }

  pub fn neighbors(&self, p: Point) -> Vec<Point> {
    [(-1, 0), (1, 0), (0, -1), (0, 1)]
      .into_iter()
      .map(|(dx, dy)| (p.0 + dx, p.1 + dy))
      .filter(|p2| self.points.contains_key(&p2))
      .collect()
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn part1_example() {
    let g : Grid = SAMPLE.parse().expect("shit");
    assert_eq!(g.lowest_risk_path(), 40);
  }

  #[test]
  fn part1_solution() {
    let g : Grid = INPUT.parse().expect("shit");
    assert_eq!(g.lowest_risk_path(), 702);
  }

  #[test]
  fn part2_example() {
    let mut g : Grid = SAMPLE.parse().expect("shit");
    g.explode();
    assert_eq!(g.lowest_risk_path(), 315);
  }

  #[test]
  fn part2_solution() {
    let mut g : Grid = INPUT.parse().expect("shit");
    g.explode();
    assert_eq!(g.lowest_risk_path(), 2955);
  }
}
