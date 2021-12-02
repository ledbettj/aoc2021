const INPUT: &'static str = include_str!("../inputs/day2.txt");

enum Command {
  Forward(isize),
  Down(isize),
  Up(isize),
}

type Position = (isize, isize);
type Position3 = (isize, isize, isize);

#[derive(Debug, Clone)]
struct CommandParseError(&'static str);

impl TryFrom<&str> for Command {
  type Error = CommandParseError;

  fn try_from(value: &str) -> Result<Self, Self::Error> {
    let mut parts = value.split_ascii_whitespace();
    let cmd = parts.next().ok_or(CommandParseError("No direction"))?;

    let amount: isize = parts
      .next()
      .ok_or(CommandParseError("No amount"))?
      .parse()
      .map_err(|_| CommandParseError("Invalid amount"))?;

    match cmd {
      "forward" => Ok(Command::Forward(amount)),
      "down" => Ok(Command::Down(amount)),
      "up" => Ok(Command::Up(amount)),
      _ => Err(CommandParseError("Unknown direction")),
    }
  }
}

impl Command {
  fn load_list<'a, I: Iterator<Item = &'a str>>(
    lines: I,
  ) -> Result<Vec<Command>, CommandParseError> {
    lines.map(|line| line.try_into()).collect()
  }

  fn exec(&self, pos: Position) -> Position {
    match self {
      Command::Forward(n) => (pos.0 + n, pos.1),
      Command::Down(n) => (pos.0, pos.1 + n),
      Command::Up(n) => (pos.0, pos.1 - n),
    }
  }

  fn exec_aim(&self, pos: Position3) -> Position3 {
    match self {
      Command::Forward(n) => (pos.0 + n, pos.1 + pos.2 * n, pos.2),
      Command::Down(n) => (pos.0, pos.1, pos.2 + n),
      Command::Up(n) => (pos.0, pos.1, pos.2 - n),
    }
  }

  fn exec_list(initial: (isize, isize), list: &[Command]) -> (isize, isize) {
    list.iter().fold(initial, |pos, cmd| cmd.exec(pos))
  }

  fn exec_list_aim(initial: (isize, isize, isize), list: &[Command]) -> (isize, isize, isize) {
    list.iter().fold(initial, |pos, cmd| cmd.exec_aim(pos))
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn part1_solution() {
    let list = Command::load_list(INPUT.lines()).expect("Failed to load commands");
    let initial = (0, 0);
    let result = Command::exec_list(initial, &list);
    assert_eq!(result.0 * result.1, 2150351);
  }

  #[test]
  fn part2_solution() {
    let list = Command::load_list(INPUT.lines()).expect("Failed to load commands");
    let initial = (0, 0, 0);
    let result = Command::exec_list_aim(initial, &list);
    assert_eq!(result.0 * result.1, 1842742223);
  }
}
