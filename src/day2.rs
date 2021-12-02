const INPUT : &'static str = include_str!("../inputs/day2.txt");

enum Command {
  Forward(isize),
  Down(isize),
  Up(isize)
}

impl From<&str> for Command {

  fn from(value: &str) -> Self {
    let mut parts = value.split_ascii_whitespace();
    let cmd = parts.next().unwrap();
    let amount : isize = parts.next().unwrap().parse().unwrap();
    match cmd {
      "forward" => Command::Forward(amount),
      "down" => Command::Down(amount),
      "up" => Command::Up(amount),
      _ => panic!("unexpected command")
    }
  }
}

impl Command {
  fn load_list<'a, I: Iterator<Item=&'a str>>(lines: I) -> Vec<Command> {
    lines.map(|line| line.into()).collect()
  }

  fn exec(&self, pos: (isize, isize)) -> (isize, isize) {
    match self {
      Command::Forward(n) => (pos.0 + n, pos.1),
      Command::Down(n) => (pos.0, pos.1 + n),
      Command::Up(n) => (pos.0, pos.1 - n)
    }
  }

  fn exec_list(initial: (isize, isize), list: &[Command]) -> (isize, isize) {
    list.iter().fold(initial, |pos, cmd|{
      cmd.exec(pos)
    })
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn part1_example() {

  }

  #[test]
  fn part1_solution() {
    let list = Command::load_list(INPUT.lines());
    let initial = (0, 0);
    let result = Command::exec_list(initial, &list);
    assert_eq!(result.0 * result.1, 0);
  }

  #[test]
  fn part2_example() {

  }

  #[test]
  fn part2_solution() {

  }
}