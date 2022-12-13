//extern crate nom;
use nom::{
    branch::alt,
    character::complete::{char, digit1},
    combinator::{map, map_res},
    multi::separated_list0,
    sequence::delimited,
    IResult,
};
type Input = String;

pub struct Day13 {
    input: Input,
}

impl Day13 {
    pub fn parse(input: &str) -> Day13 {
        Day13 {
            input: input.to_string(),
        }
    }

    pub fn part1(&self) -> usize {
        0
    }

    pub fn part2(&self) -> usize {
        0
    }
}

#[derive(PartialEq, Debug)]
enum Packet {
    Integer(usize),
    List(Vec<Packet>),
}

fn packet(input: &str) -> IResult<&str, Packet> {
    Ok(alt((
        delimited(
            char('['),
            map(separated_list0(char(','), packet), |p| Packet::List(p)),
            char(']'),
        ),
        map(digit1, |out: &str| Packet::Integer(out.parse().unwrap())),
    ))(input)?)
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &'static str = "";

    #[test]
    fn parser() {
        let t = "[123,456]";
        assert_eq!(
            packet(t),
            Ok((
                "",
                Packet::List(vec![Packet::Integer(123), Packet::Integer(456)])
            ))
        );
        let t = "[]";
        assert_eq!(packet(t), Ok(("", Packet::List(vec![]))));
        let t = "[1,[1,2,[]]]";
        assert_eq!(
            packet(t),
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
        assert_eq!(parse.part1(), 0);
        assert_eq!(parse.part2(), 0);
    }
}
