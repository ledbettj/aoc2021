
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

fn bit_tallies(input: &[&str]) -> Vec<[usize; 2]> {
  let mut tallies : Vec<[usize; 2]> = vec![];
  for _ in 0..input[0].len() {
    tallies.push([0, 0]);
  }

  for line in input {
    for (index, ch) in line.chars().enumerate() {
      let zero_or_one : usize = ch.to_digit(10).unwrap() as usize;
      tallies.get_mut(index).unwrap()[zero_or_one] += 1;
    }
  }

  tallies
}

fn calc_gamma_epsilon(input: &[&str]) -> (usize, usize) {
  let tallies = bit_tallies(&input);
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

fn calc_oxygen(input: &[&str]) -> usize {
  let mut values : Vec<Vec<char>> = input.iter().map(|s| s.chars().collect()).collect();

  for pos in 0..input[0].len() {
    let ones = values.iter().filter(|s| s[pos] == '1').count();
    let zeros = values.len() - ones;
    let ch = if ones >= zeros { '1' } else { '0' };
    values = values.iter().filter(|s| s[pos] == ch).cloned().collect();

    if values.len() == 1 {
      break;
    }
  }

  usize::from_str_radix(&values[0].iter().collect::<String>(), 2).unwrap()
}


fn calc_co2(input: &[&str]) -> usize {
  let mut values : Vec<Vec<char>> = input.iter().map(|s| s.chars().collect()).collect();

  for pos in 0..input[0].len() {
    let ones = values.iter().filter(|s| s[pos] == '1').count();
    let zeros = values.len() - ones;
    let ch = if ones >= zeros { '0' } else { '1' };
    values = values.iter().filter(|s| s[pos] == ch).cloned().collect();

    if values.len() == 1 {
      break;
    }
  }

  usize::from_str_radix(&values[0].iter().collect::<String>(), 2).unwrap()
}


#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn part1_example() {
    assert_eq!(calc_gamma_epsilon(&SAMPLE), (22, 9));
  }

  #[test]
  fn part1_solution() {
    let (g, e) = calc_gamma_epsilon(&input());
    assert_eq!(g * e, 1092896);
  }

  #[test]
  fn part2_example() {
    let o = calc_oxygen(&SAMPLE);
    let co2 = calc_co2(&SAMPLE);

    assert_eq!((o, co2), (23, 10));
  }

  #[test]
  fn part2_solution() {
    let o = calc_oxygen(&input());
    let co2 = calc_co2(&input());
    assert_eq!(o * co2, 4672151);
  }
}
