const INPUT: &'static str = include_str!("../inputs/day1.txt");
const SAMPLE: &[usize] = &[199, 200, 208, 210, 200, 207, 240, 269, 260, 263];

fn sonar_readings() -> Vec<usize> {
  INPUT
    .lines()
    .map(|line| line.parse())
    .collect::<Result<Vec<usize>, _>>()
    .expect("Failed to parse readings to numbers")
}

fn increasing_count(readings: &[usize]) -> usize {
  readings.windows(2).filter(|pair| pair[1] > pair[0]).count()
}

fn window_increasing_count(readings: &[usize]) -> usize {
  let windows: Vec<usize> = readings.windows(3).map(|set| set.iter().sum()).collect();

  increasing_count(&windows)
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn part1_example() {
    assert_eq!(increasing_count(SAMPLE), 7);
  }

  #[test]
  fn part1_solution() {
    assert_eq!(increasing_count(&sonar_readings()), 1557);
  }

  #[test]
  fn part2_example() {
    assert_eq!(window_increasing_count(SAMPLE), 5);
  }

  #[test]
  fn part2_solution() {
    assert_eq!(window_increasing_count(&sonar_readings()), 1608);
  }
}
