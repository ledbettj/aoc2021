
const INPUT : &'static str = include_str!("../inputs/day3.txt");
const SAMPLE : &[&'static str] = &[
  "00100",
  "11110",
  "10110",
  "10111",
  "10101",
  "01111",
  "00111",
  "11100",
  "10000",
  "11001",
  "00010",
  "01010"
];

fn input() -> Vec<&'static str> {
  INPUT.lines().collect()
}

fn calc_rates(input: &[&str]) -> (usize, usize) {
  let mut tallies : Vec<[usize; 2]> = vec![];
  for i in 0..input[0].len() {
    tallies.push([0, 0]);
  }

  for line in input {
    for (index, ch) in line.chars().enumerate() {
      let zero_or_one : usize = ch.to_digit(10).unwrap() as usize;
      tallies.get_mut(index).unwrap()[zero_or_one] += 1;
    }
  }

  let mut gamma = 0;
  let mut epsilon = 0;

  for row in tallies.iter() {
    gamma <<= 1;
    epsilon <<= 1;
    if row[1] > row[0] {
      gamma |= 1;
    } else {
      epsilon |= 1;
    }
  }

  (gamma, epsilon)
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn part1_example() {
    assert_eq!(calc_rates(&SAMPLE), (22, 9));
  }

  #[test]
  fn part1_solution() {
    let (g, e) = calc_rates(&input());
    assert_eq!(g * e, 1092896);
  }

  #[test]
  fn part2_example() {

  }

  #[test]
  fn part2_solution() {

  }
}
