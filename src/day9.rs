use std::collections::{HashMap, HashSet};
use std::str::FromStr;

const INPUT : &'static str = include_str!("../inputs/day9.txt");
const SAMPLE : &'static str = include_str!("../inputs/day9.sample.txt");

type Point = (isize, isize);

#[derive(Clone)]
struct Grid {
  points: HashMap<Point, isize>
}

impl FromStr for Grid {
  type Err = &'static str;

  fn from_str(s: &str) -> Result<Self, Self::Err> {
    let points = s
      .trim()
      .lines()
      .enumerate()
      .flat_map(|(y, row)| {
        row
          .chars()
          .enumerate()
          .map(move |(x, ch)|{
            ch.to_digit(10)
              .map(|digit| ((x as isize, y as isize), digit as isize))
              .ok_or("Invalid character")
          })
      })
      .collect::<Result<HashMap<Point, isize>, Self::Err>>()?;

      Ok(Grid { points })
  }
}

impl Grid {
  fn low_points_score(&self) -> isize {
    self
      .low_points()
      .filter_map(|pos| self.height_at(pos))
      .map(|h| h + 1)
      .sum()
  }

  fn low_points(&self) -> impl Iterator<Item=&Point> {
    self
      .points
      .keys()
      .filter(|pos| self.is_low_point(pos))
  }

  fn is_low_point(&self, pos: &Point) -> bool {
    let height = self.height_at(pos).expect("Cannot check if missing point is low point");

    self.neighbors(pos)
      .filter_map(|p| self.height_at(&p))
      .all(|h| height < h)
  }

  fn neighbors(&self, pos: &Point) -> impl Iterator<Item=Point> {
    let offsets = &[(-1, 0), (1, 0), (0, -1), (0, 1)];
    let &(x, y) = pos;
    offsets
      .iter()
      .map(move |(dx, dy)| (x + dx, y + dy))
  }

  fn height_at(&self, pos: &Point) -> Option<isize> {
    self.points.get(pos).map(|h| *h)
  }

  fn basin_sizes(&self) -> Vec<usize> {
    let mut seen : HashSet<Point> = HashSet::new();
    let mut sizes : Vec<usize> = vec![];

    loop {
      let mut current : usize = 0;
      let mut to_visit = vec![];

      let start = self.points
        .keys()
        .find(|pos| !seen.contains(pos) && self.height_at(pos).unwrap() != 9);

      match start {
        None => break,
        Some(&start_pos) => {
          to_visit.push(start_pos);
          seen.insert(start_pos);

          while let Some(pos) = to_visit.pop() {
            current += 1;

            for np in self.neighbors(&pos) {
              if !seen.insert(np) {
                continue;
              }

              match self.height_at(&np) {
                Some(h) if h != 9 => { to_visit.push(np); },
                _ => {}
              };
            }
        }
      }

      }
      sizes.push(current);
    }

    sizes
  }

  fn largest_basins(&self, count: usize) -> Vec<usize> {
    let mut sizes = self.basin_sizes();

    sizes.sort();
    sizes.reverse();
    sizes
      .iter()
      .take(count)
      .cloned()
      .collect()
  }
}


#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn part1_example() {
    let g : Grid = SAMPLE.parse().expect("shit");
    assert_eq!(g.low_points_score(), 15);
  }

  #[test]
  fn part1_solution() {
    let g : Grid = INPUT.parse().expect("shit");
    assert_eq!(g.low_points_score(), 594);
  }

  #[test]
  fn part2_example() {
    let g : Grid = SAMPLE.parse().expect("shit");
    assert_eq!(g.largest_basins(3), vec![14, 9, 9]);
  }

  #[test]
  fn part2_solution() {
    let g : Grid = INPUT.parse().expect("shit");
    let b = g.largest_basins(3);

    let ans : usize = b.into_iter().reduce(|a, z| (a * z)).unwrap();

    assert_eq!(ans, 858494);
  }
}
