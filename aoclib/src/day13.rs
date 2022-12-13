//extern crate nom;
use nom::{
    branch::alt,
    character::complete::{char, digit1},
    combinator::{map, map_res},
    multi::separated_list0,
    sequence::delimited,
    IResult,
};

#[derive(PartialEq, Debug, Clone)]
enum Packet {
    Integer(usize),
    List(Vec<Packet>),
}

type Input = Vec<(Packet, Packet)>;

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

    fn compare(&self, other: &Self) -> Option<bool> {
        match (self, other) {
            (Packet::Integer(v1), Packet::Integer(v2)) => if v1 == v2 { None } else { Some(v1 < v2) }
            (Packet::Integer(v1), Packet::List(_)) => Packet::List(vec![Packet::Integer(*v1)]).compare(other),
            (Packet::List(_), Packet::Integer(v2)) => self.compare(&Packet::List(vec![Packet::Integer(*v2)])),
            (Packet::List(v1), Packet::List(v2)) => {
                let ordered = std::iter::zip(v1.iter(), v2.iter())
                    .fold(None, |acc, (a,b)| 
                        match acc {
                            Some(_) => acc,
                            None => a.compare(b)
                        }
                    );
                if v1.len() < v2.len() && ordered.is_none() {
                    Some(true)
                } else if v1.len() > v2.len() && ordered.is_none() {
                    Some(false)
                } else {
                    ordered
                }
            },
            _ => panic!()
        }
    }
}

pub struct Day13 {
    input: Input,
}

impl Day13 {
    pub fn parse(input: &str) -> Day13 {
        Day13 {
            input: input.split("\n\n").map(|ll| {
                let (p1, p2) = ll.split_once('\n').unwrap();
                (Packet::parse(p1).unwrap().1, Packet::parse(p2).unwrap().1)
            }).collect(),
        }
    }

    pub fn part1(&self) -> usize {
        self.input
            .iter()
            .enumerate()
            .inspect(|(i,(a,b))| println!("{} {}",i,a.compare(b).unwrap()))
            .filter(|(_i,(a,b))| a.compare(b).unwrap())
            .map(|(i,_)| i+1)
            .sum()
    }

    pub fn part2(&self) -> usize {
        0
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
        println!("{:?}", parse.input);
        assert_eq!(parse.part1(), 0);
        assert_eq!(parse.part2(), 0);
    }
}
