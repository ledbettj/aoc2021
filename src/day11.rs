use std::str::FromStr;
use std::collections::{HashMap, HashSet};

const INPUT : &'static str = include_str!("../inputs/day11.txt");
const SAMPLE : &'static str = include_str!("../inputs/day11.sample.txt");

type Point = (isize, isize);

#[derive(Clone)]
struct Grid {
  cells: HashMap<Point, usize>
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
            ch.to_digit(10)
              .ok_or("Cant parse digit")
              .map(|digit| ((x as isize, y as isize), digit as usize))
          })
      })
      .collect::<Result<HashMap<Point, usize>, Self::Err>>()
      .map(|cells| Grid { cells })
  }
}

impl Grid {
  fn step(&mut self) -> usize {
    // set of points that flashed this iteration
    let mut seen : HashSet<Point> = HashSet::new();

    // first increment all cells
    self
      .cells
      .iter_mut()
      .for_each(|(_, value)| *value += 1 );

    loop {
      // find cells that should flash
      let flashes : Vec<Point> = self
        .cells
        .iter()
        .filter(|(point, &value)| !seen.contains(point) && value > 9)
        .map(|(point, _)| point)
        .cloned()
        .collect::<Vec<Point>>();

      // mark as seen so we don't double flash
      for f in &flashes {
        seen.insert(*f);
      }

      // find neighbors of flashes
      let to_increment = flashes
        .iter()
        .flat_map(|&point| self.neighbors(point))
        .collect::<Vec<Point>>();

      // no neighbors to increment means we're done
      if to_increment.is_empty() {
        break;
      }

      // increment neighbors
      for point in to_increment {
        self.cells
          .get_mut(&point)
          .map(|v| *v += 1);
      }
    }

    // set all cells which flashed to zero
    for pos in &seen {
      self.cells.get_mut(&pos).map(|v| *v = 0);
    }

    // total number of flashes this iteration
    seen.len()
  }

  fn run(&mut self, steps: usize) -> usize {
    (0..steps).map(|_| self.step()).sum()
  }

  fn run_until_sync(&mut self) -> Option<usize> {
    (1..usize::MAX).find(|_| self.step() == self.cells.len())
  }

  fn neighbors<'a>(&'a self, pos: Point) -> impl Iterator<Item=Point> + 'a {
    [(-1, -1), (-1, 0), (-1, 1), (0, -1), (0, 1), (1, -1), (1, 0), (1, 1)]
      .into_iter()
      .map(move |(dx, dy)| (pos.0 + dx, pos.1 + dy))
      .filter(|p| self.cells.contains_key(&p))
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn part1_example() {
    let mut g : Grid = SAMPLE.parse().expect("shit");
    assert_eq!(g.run(100), 1656);
  }

  #[test]
  fn part1_solution() {
    let mut g : Grid = INPUT.parse().expect("shit");
    assert_eq!(g.run(100), 1702);
  }

  #[test]
  fn part2_example() {
    let mut g : Grid = SAMPLE.parse().expect("shit");
    assert_eq!(g.run_until_sync(), Some(195));
  }

  #[test]
  fn part2_solution() {
    let mut g : Grid = INPUT.parse().expect("shit");
    assert_eq!(g.run_until_sync(), Some(251));
  }
}
