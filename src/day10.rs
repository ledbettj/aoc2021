const INPUT : &'static str = include_str!("../inputs/day10.txt");
const SAMPLE : &'static str = include_str!("../inputs/day10.sample.txt");

fn parse(line: &str) -> Result<String, char> {
  let mut stack = vec![];

  for ch in line.chars() {
    match ch {
      '[' | '{' | '<' | '(' => stack.push(ch),
      ']' | '}' | '>' | ')' => {
        let opening = stack.pop();
        match (opening, ch) {
          (Some('['), ']') | (Some('{'), '}') |
          (Some('<'), '>') | (Some('('), ')') => Ok(()),
          (None, _) => Err(ch), // closing without any opening
          (_, _) => Err(ch) // mismatch
        }?;
      },
      _ => panic!("Invalid character: {}", ch)
    };
  }
  let completion = stack
    .iter()
    .rev()
    .map(|ch| match ch {
      '(' => ')',
      '[' => ']',
      '{' => '}',
      '<' => '>',
      _   => panic!("{} not a valid input", ch)
    })
    .collect::<String>();

  Ok(completion)
}

fn error_score(errs: &[char]) -> usize {
  errs
    .iter()
    .map(|ch| match ch {
      ')' => 3,
      ']' => 57,
      '}' => 1197,
      '>' => 25137,
      _   => panic!("{} not an error", ch)
    })
    .sum()
}

fn completion_score(completion: &str) -> usize {
  completion
    .chars()
    .map(|ch| match ch {
      ')' => 1,
      ']' => 2,
      '}' => 3,
      '>' => 4,
      _ => panic!("{} still not valid", ch)
    })
    .fold(0, |total, char_score| total * 5 + char_score)
}

fn completions_middle_score(incomplete: &[String]) -> usize {
  let mut scores = incomplete
    .iter()
    .map(|completion| completion_score(&completion))
    .collect::<Vec<usize>>();

  scores.sort();
  scores[scores.len() / 2]
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn part1_example() {
    let errs = SAMPLE
      .lines()
      .filter_map(|line| parse(line).err())
      .collect::<Vec<char>>();

    assert_eq!(&errs, &['}', ')', ']', ')', '>']);
    assert_eq!(error_score(&errs), 26397);
  }

  #[test]
  fn part1_solution() {
    let errs = INPUT
      .lines()
      .filter_map(|line| parse(line).err())
      .collect::<Vec<char>>();

    assert_eq!(error_score(&errs), 323691);
  }

  #[test]
  fn part2_example() {
    let incomplete = SAMPLE
      .lines()
      .filter_map(|line| parse(line).ok())
      .collect::<Vec<String>>();

    assert_eq!(&incomplete, &[
      "}}]])})]",
      ")}>]})",
      "}}>}>))))",
      "]]}}]}]}>",
      "])}>",
    ]);

    assert_eq!(completions_middle_score(&incomplete), 288957);
  }

  #[test]
  fn part2_solution() {
    let incomplete = INPUT
      .lines()
      .filter_map(|line| parse(line).ok())
      .collect::<Vec<String>>();

    assert_eq!(completions_middle_score(&incomplete), 2858785164);

  }
}
