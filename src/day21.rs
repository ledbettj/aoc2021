use std::collections::HashMap;

const INPUT : &'static str = include_str!("../inputs/day21.txt");
const ROLLS : &[[usize; 3]] = &[
  [1, 1, 1], [1, 1, 2], [1, 1, 3], [1, 2, 1], [1, 2, 2], [1, 2, 3], [1, 3, 1], [1, 3, 2], [1, 3, 3],
  [2, 1, 1], [2, 1, 2], [2, 1, 3], [2, 2, 1], [2, 2, 2], [2, 2, 3], [2, 3, 1], [2, 3, 2], [2, 3, 3],
  [3, 1, 1], [3, 1, 2], [3, 1, 3], [3, 2, 1], [3, 2, 2], [3, 2, 3], [3, 3, 1], [3, 3, 2], [3, 3, 3]
];

#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Hash)]
struct Dirac {
  player: usize,
  spaces: [usize; 2],
  scores: [usize; 2],
}

impl Dirac {
  fn new(p1: usize, p2: usize) -> Self {
    Self { spaces: [p1 - 1, p2 - 1], .. Default::default() }
  }

  fn turn(&mut self, rolls: &[usize]) -> &Self {
    let player = self.player;
    let advance : usize = rolls.iter().sum();
    self.spaces[player] += advance;
    self.spaces[player] %= 10;
    self.scores[player] += self.spaces[player] + 1;
    self.player = (self.player + 1) % 2;

    self
  }

  fn is_over(&self, limit: usize) -> bool {
    self.winner(limit).is_some()
  }

  fn winner(&self, limit: usize) -> Option<usize> {
    self
      .scores
      .iter()
      .enumerate()
      .find(|&(_, score)| *score >= limit)
      .map(|(index, _)| index)
  }

  fn quantum_turn(&self) -> impl Iterator<Item=Dirac> + '_  {
    ROLLS
      .iter()
      .map(|roll| *self.clone().turn(roll))
  }
}

struct QuantumDirac {
  states: HashMap<Dirac, u64>
}

impl QuantumDirac {
  fn new(d: Dirac) -> Self {
    let mut states = HashMap::new();
    states.insert(d, 1u64);
    Self { states }
  }

  fn run_until(&mut self, limit: usize) {
    while !self.states.keys().all(|d| d.is_over(limit)) {
      let mut next = HashMap::new();

      for (board, count) in self.states.iter() {
        if board.is_over(limit) {
          let e = next.entry(*board).or_insert(0u64);
          *e += count;
        } else {
          for next_board in board.quantum_turn() {
            let e = next.entry(next_board).or_insert(0u64);
            *e += count;
          }
        }
      }
      self.states = next;
    };
  }

  fn wins(&self, limit: usize) -> [u64; 2] {
    let mut w = [0u64, 0u64];

    self
      .states
      .iter()
      .for_each(|(board, count)|{
        if let Some(player) = board.winner(limit) {
          w[player] += count;
        };

      });

    w
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn part1_example() {
    let mut d = Dirac::new(4, 8);
    let mut dice = (1..1_000_000).into_iter();
    let mut turns = 0;
    while !d.is_over(1_000) {
      let roll = [
        dice.next().unwrap(),
        dice.next().unwrap(),
        dice.next().unwrap()
      ];
      d.turn(&roll);
      turns += 1;
    }
    assert_eq!(d.scores, [1_000, 745]);
    assert_eq!(turns * 3, 993);
    assert_eq!(d.scores.iter().min().unwrap() * turns * 3, 739785);
  }

  #[test]
  fn part1_solution() {
    let mut d = Dirac::new(2, 8);
    let mut dice = (1..1_000_000).into_iter();
    let mut turns = 0;

    while !d.is_over(1_000) {
      let roll = [
        dice.next().unwrap(),
        dice.next().unwrap(),
        dice.next().unwrap()
      ];
      d.turn(&roll);
      turns += 1;

    }
    assert_eq!(d.scores.iter().min().unwrap() * turns * 3, 1196172);
  }

  #[test]
  fn part2_example() {
    let d = Dirac::new(4, 8);
    let mut q = QuantumDirac::new(d);

    q.run_until(21);
    assert_eq!(q.wins(21).iter().max(), Some(&444356092776315u64));
  }

  #[test]
  fn part2_solution() {
    let d = Dirac::new(2, 8);
    let mut q = QuantumDirac::new(d);

    q.run_until(21);
    assert_eq!(q.wins(21).iter().max(), Some(&106768284484217u64));
  }
}
