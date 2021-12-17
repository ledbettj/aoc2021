use std::ops::RangeInclusive;

struct Target {
  x: RangeInclusive<isize>,
  y: RangeInclusive<isize>
}

impl Target {
  fn new(x: RangeInclusive<isize>, y: RangeInclusive<isize>) -> Self {
    Target { x, y }
  }

  fn highest_point(&self) -> Option<isize> {
    // min dx is requires (1...dx).sum() >= target.x.start
    // max dx is target.end
    let x_range = (0..=*self.x.end())
      .filter(|&n| self.x.contains(&(n * (n + 1) / 2)));

    let t = &self;

    x_range
      .flat_map(|dx|{
        (0..10_000).filter_map(move |dy| t.highest_point_for(dx, dy))
      })
      .max()
  }

  fn hit_count(&self) -> usize {
    // min dx is requires (1...dx).sum() >= target.x.start
    // max dx is target.end
    let x_range = (0..=*self.x.end())
      .filter(|&n| n * (n + 1) / 2 >= *self.x.start())
      .collect::<Vec<isize>>();
    // min y is lowest y on target
    let y_start = *self.y.start();
    let t = &self;

    x_range
      .iter()
      .flat_map(|&dx|{
        (y_start..10_000).filter_map(move |dy| t.highest_point_for(dx, dy))
      })
      .count()
  }

  fn highest_point_for(&self, mut dx: isize, mut dy: isize) -> Option<isize> {
    let mut x = 0;
    let mut y = 0;
    let mut highest = y;

    loop {
      if self.x.contains(&x) && self.y.contains(&y) {
        return Some(highest);
      }
      // too much x
      if x > *self.x.end() {
        return None;
      }
      // fell too far
      if y < *self.y.start() {
        return None;
      }

      x += dx;
      y += dy;
      highest = y.max(highest);

      if dx > 0 {
        dx -= 1;
      }
      dy -= 1;
    }
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn part1_example() {
    let t = Target::new(20..=30, -10..=-5);
    assert_eq!(t.highest_point(), Some(45));
  }

  #[test]
  fn part1_solution() {
    let t = Target::new(119..=176, -141..=-84);
    assert_eq!(t.highest_point(), Some(9870));
  }

  #[test]
  fn part2_example() {
    let t = Target::new(20..=30, -10..=-5);
    assert_eq!(t.hit_count(), 112);

  }

  #[test]
  fn part2_solution() {
    let t = Target::new(119..=176, -141..=-84);
    assert_eq!(t.hit_count(), 5523);
  }
}
