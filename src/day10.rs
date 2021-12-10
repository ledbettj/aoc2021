use std::collections::HashMap;

const INPUT : &'static str = include_str!("../inputs/day10.txt");
const SAMPLE : &'static str = include_str!("../inputs/day10.sample.txt");

#[derive(Clone, Copy)]
struct Chunk {
  open: char,
  close: char,
  error_score: usize,
  completion_score: usize
}

impl Chunk {
  pub fn new(open: char, close: char, error_score: usize, completion_score: usize) -> Self {
    Self { open, close, error_score, completion_score }
  }
  pub fn is_close(&self, ch: char) -> bool {
    self.close == ch
  }
}

lazy_static! {
  // map of opening char to chunk
  static ref OPENING : HashMap<char, Chunk> = [
    Chunk::new('(', ')', 3, 1),    Chunk::new('[', ']', 57, 2),
    Chunk::new('{', '}', 1197, 3), Chunk::new('<', '>', 25137, 4),
  ].into_iter().map(|chunk| (chunk.open, chunk)).collect();
  // map of closing char to chunk
  static ref CLOSING : HashMap<char, Chunk> = OPENING.values().map(|&chunk| (chunk.close, chunk)).collect();
}

// parse a single line.  If it is incomplete, returns Ok(completion).
// otherwise returns Err(mismatched_closing_char)
fn parse(line: &str) -> Result<String, char> {
  let incomplete = line
    .chars()
    .try_fold(vec![], |mut stack, ch| match ch {
        '[' | '{' | '<' | '(' => {
          stack.push(ch);
          Ok(stack)
        },
        ']' | '}' | '>' | ')' if !stack.is_empty() => {
          let o = stack.pop().unwrap();
          if OPENING.get(&o).unwrap().is_close(ch) { Ok(stack) } else { Err(ch) }
        },
        _ if stack.is_empty() => panic!("Closing character without opening"),
        _ => panic!("Invalid character: {}", ch)
    })?;

  // map remaining opened chunks to their closing characters
  let completion : String = incomplete
    .iter()
    .rev()
    .map(|ch| OPENING.get(ch).unwrap().close)
    .collect();

  Ok(completion)
}

fn parse_lines<'a>(input: &'a &str) -> impl Iterator<Item=Result<String, char>> + 'a {
  input.lines().map(parse)
}

// given list of errors, return the total score
fn error_score(errs: &[char]) -> usize {
  errs.iter().map(|ch| CLOSING.get(ch).unwrap().error_score).sum()
}

// given a single completion, calculate its score
fn completion_score(completion: &str) -> usize {
  completion
    .chars()
    .map(|ch| CLOSING.get(&ch).unwrap().completion_score)
    .fold(0, |total, char_score| total * 5 + char_score)
}

// given a list of completions, calculate the middle score
fn completions_middle_score(completions: &[String]) -> usize {
  let mut scores : Vec<usize> = completions.iter().map(|c| completion_score(&c)).collect();
  scores.sort();

  scores[scores.len() / 2]
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn part1_example() {
    let errs : Vec<char> = parse_lines(&SAMPLE).filter_map(|r| r.err()).collect();

    assert_eq!(&errs, &['}', ')', ']', ')', '>']);
    assert_eq!(error_score(&errs), 26397);
  }

  #[test]
  fn part1_solution() {
    let errs : Vec<char> = parse_lines(&INPUT).filter_map(|r| r.err()).collect();

    assert_eq!(error_score(&errs), 323691);
  }

  #[test]
  fn part2_example() {
    let incomplete : Vec<String> = parse_lines(&SAMPLE).filter_map(|r| r.ok()).collect();

    assert_eq!(&incomplete, &["}}]])})]", ")}>]})", "}}>}>))))", "]]}}]}]}>", "])}>"]);
    assert_eq!(completions_middle_score(&incomplete), 288957);
  }

  #[test]
  fn part2_solution() {
    let incomplete : Vec<String> = parse_lines(&INPUT).filter_map(|r| r.ok()).collect();

    assert_eq!(completions_middle_score(&incomplete), 2858785164);
  }
}
