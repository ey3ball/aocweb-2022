use nom::{
    branch::alt,
    character::complete::{char, digit1},
    combinator::{map, map_res},
    multi::separated_list0,
    sequence::delimited,
    IResult,
};
use std::cmp::Ordering;

#[derive(Eq, PartialEq, Debug, Clone)]
enum Packet {
    Integer(usize),
    List(Vec<Packet>),
}

type Input = Vec<Packet>;

impl Packet {
    fn parse(input: &str) -> IResult<&str, Packet> {
        Ok(alt((
            delimited(
                char('['),
                map(separated_list0(char(','), Self::parse), |p| Packet::List(p)),
                char(']'),
            ),
            map(digit1, |out: &str| Packet::Integer(out.parse().unwrap())),
        ))(input)?)
    }

    fn compare(&self, other: &Self) -> Option<std::cmp::Ordering> {
        match (self, other) {
            (Packet::Integer(v1), Packet::Integer(v2)) => {
                if v1 == v2 {
                    None
                } else {
                    Some(v1.cmp(v2))
                }
            }
            (Packet::Integer(v1), Packet::List(_)) => {
                Packet::List(vec![Packet::Integer(*v1)]).compare(other)
            }
            (Packet::List(_), Packet::Integer(v2)) => {
                self.compare(&Packet::List(vec![Packet::Integer(*v2)]))
            }
            (Packet::List(v1), Packet::List(v2)) => {
                let ordered =
                    std::iter::zip(v1.iter(), v2.iter()).fold(None, |acc, (a, b)| match acc {
                        Some(_) => acc,
                        None => a.compare(b),
                    });
                if v1.len() < v2.len() && ordered.is_none() {
                    Some(Ordering::Less)
                } else if v1.len() > v2.len() && ordered.is_none() {
                    Some(Ordering::Greater)
                } else {
                    ordered
                }
            }
            _ => panic!(),
        }
    }
}

impl Ord for Packet {
    fn cmp(&self, other: &Self) -> Ordering {
        if self == other {
            Ordering::Equal
        } else {
            self.partial_cmp(other).unwrap()
        }
    }
}

impl PartialOrd for Packet {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.compare(other)
    }
}

pub struct Day13 {
    input: Input,
}

impl Day13 {
    pub fn parse(input: &str) -> Day13 {
        let mut input: Vec<Packet> = input
            .split("\n\n")
            .flat_map(|ll| ll.lines().map(|l| Packet::parse(l).unwrap().1))
            .collect();

        input.push(Packet::parse("[[6]]").unwrap().1);
        input.push(Packet::parse("[[2]]").unwrap().1);
        Day13 { input }
    }

    pub fn part1(&self) -> usize {
        self.input
            .chunks(2)
            .enumerate()
            .filter(|(_i, cmp)| cmp[0] < cmp[1])
            .map(|(i, _)| i + 1)
            .sum()
    }

    pub fn part2(&self) -> usize {
        let mut input = self.input.clone();
        input.sort();
        let two = &self.input[self.input.len() - 1];
        let six = &self.input[self.input.len() - 2];
        let two = input.iter().position(|e| *e == *two).unwrap();
        let six = input.iter().position(|e| *e == *six).unwrap();
        (two + 1) * (six + 1)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &'static str = "[1,1,3,1,1]
[1,1,5,1,1]

[[1],[2,3,4]]
[[1],4]

[9]
[[8,7,6]]

[[4,4],4,4]
[[4,4],4,4,4]

[7,7,7,7]
[7,7,7]

[]
[3]

[[[]]]
[[]]

[1,[2,[3,[4,[5,6,7]]]],8,9]
[1,[2,[3,[4,[5,6,0]]]],8,9]";

    #[test]
    fn parser() {
        let t = "[123,456]";
        assert_eq!(
            Packet::parse(t),
            Ok((
                "",
                Packet::List(vec![Packet::Integer(123), Packet::Integer(456)])
            ))
        );
        let t = "[]";
        assert_eq!(Packet::parse(t), Ok(("", Packet::List(vec![]))));
        let t = "[1,[1,2,[]]]";
        assert_eq!(
            Packet::parse(t),
            Ok((
                "",
                Packet::List(vec![
                    Packet::Integer(1),
                    Packet::List(vec![
                        Packet::Integer(1),
                        Packet::Integer(2),
                        Packet::List(vec![])
                    ])
                ])
            ))
        );
    }

    #[test]
    fn solve() {
        let parse = Day13::parse(SAMPLE);
        assert_eq!(parse.part1(), 13);
        assert_eq!(parse.part2(), 140);
    }
}
