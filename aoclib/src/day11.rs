type Input = Vec<Monkey>;
use std::collections::VecDeque;

pub struct Day11 {
    input: Input,
}

#[derive(Debug, Clone)]
pub enum WorryOp {
    Add(usize),
    Mul(usize),
    Square,
}

#[derive(Debug, Clone)]
pub struct Monkey {
    items: VecDeque<usize>,
    worry: WorryOp,
    threshold: usize,
    dst_worry: usize,
    dst_no_worry: usize,
}

impl std::str::FromStr for Monkey {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut it = s.lines().skip(1);
        let items = it.next().ok_or("")?.split_once(": ").ok_or("")?;
        let items: VecDeque<usize> = items.1.split(',').map(|num| num.trim().parse().unwrap()).collect();
        let op = it.next().ok_or("")?.split_once(": ").ok_or("")?;
        let op = op.1.split_once('=').ok_or("")?;
        let op = if let Some(rest) = op.1.strip_prefix(" old * ") {
            if rest.ends_with("old") {
                WorryOp::Square
            } else {
                WorryOp::Mul(rest.parse().unwrap())
            }
        } else if let Some(rest) = op.1.strip_prefix(" old + ") {
            WorryOp::Add(rest.parse().unwrap())
        } else {
            panic!()
        };
        let test = it
            .next()
            .ok_or("")?
            .split(' ')
            .last()
            .unwrap()
            .parse()
            .unwrap();
        let worry = it
            .next()
            .ok_or("")?
            .split(' ')
            .last()
            .unwrap()
            .parse()
            .unwrap();
        let not_worry = it
            .next()
            .ok_or("")?
            .split(' ')
            .last()
            .unwrap()
            .parse()
            .unwrap();

        Ok(Monkey {
            items: items,
            worry: op,
            threshold: test,
            dst_worry: worry,
            dst_no_worry: not_worry,
        })
    }
}

impl std::fmt::Display for Monkey {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{:?}", self.items)
    }
}

impl WorryOp {
    pub fn exec(&self, old: usize) -> usize {
        match self {
            WorryOp::Add(x) => old + x,
            WorryOp::Mul(x) => old * x,
            WorryOp::Square => old * old
        }
    }
}

impl Monkey {
    pub fn send(&mut self, item: usize) {
        self.items.push_back(item)
    }

    pub fn inspect(&mut self) -> Vec<(usize, usize)> {
        self
            .items
            .drain(..)
            .map(|w| {
                let new_w = self.worry.exec(w) / 3;
                if new_w % self.threshold == 0 {
                    (self.dst_worry, new_w)
                } else {
                    (self.dst_no_worry, new_w)
                }
            })
            .collect()
    }
}

impl Day11 {
    pub fn parse(input: &str) -> Day11 {
        Day11 {
            input: input
                .split("\n\n")
                .map(|monkey| monkey.parse::<Monkey>().unwrap())
                .collect(),
        }
    }

    pub fn part1(&self) -> usize {
        let mut state = self.input.clone();
        let mut business: Vec<usize> = vec![0; state.len()];
        for _round in 0..=19 {
            println!("round {}", _round);
            for m in 0..state.len() {
                println!("\tm{} {}", m, state[m]);
            }
            println!("\tbusiness {:?}", business);
            for m in 0..state.len() {
                let sent = state[m].inspect();
                business[m] += sent.len();
                sent.iter().for_each(|&(target, worry)| state[target].send(worry))
            }
        }
        business.sort();
        business.iter().rev().take(2).product()
    }

    pub fn part2(&self) -> usize {
        0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &'static str = "Monkey 0:
  Starting items: 79, 98
  Operation: new = old * 19
  Test: divisible by 23
    If true: throw to monkey 2
    If false: throw to monkey 3

Monkey 1:
  Starting items: 54, 65, 75, 74
  Operation: new = old + 6
  Test: divisible by 19
    If true: throw to monkey 2
    If false: throw to monkey 0

Monkey 2:
  Starting items: 79, 60, 97
  Operation: new = old * old
  Test: divisible by 13
    If true: throw to monkey 1
    If false: throw to monkey 3

Monkey 3:
  Starting items: 74
  Operation: new = old + 3
  Test: divisible by 17
    If true: throw to monkey 0
    If false: throw to monkey 1";

    #[test]
    fn solve() {
        let parse = Day11::parse(SAMPLE);
        println!("{:?}", parse.input);
        assert_eq!(parse.part1(), 0);
        assert_eq!(parse.part2(), 0);
    }
}
