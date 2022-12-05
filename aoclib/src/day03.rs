use std::collections::HashSet;

type Input = String;
type RuckSack = (HashSet<char>, HashSet<char>);

pub struct Day03 {
    input: Vec<RuckSack>,
}

impl Day03 {
    pub fn parse(input: &str) -> Day03 {
        let parsed = input
            .lines()
            .map(|l| {
                let chars: Vec<char> = l.chars().collect();
                (
                    chars[0..chars.len() / 2].iter().copied().collect(),
                    chars[chars.len() / 2..].iter().copied().collect(),
                )
            })
            .collect();

        Day03 { input: parsed }
    }

    pub fn priority(c: char) -> usize {
        if c >= 'A' && c <= 'Z' {
            (c as u8 - b'A') as usize + 27
        } else {
            (c as u8 - b'a') as usize + 1
        }
    }

    pub fn part1(&self) -> usize {
        let shared: Vec<char> = self
            .input
            .iter()
            .map(|(c1, c2)| *(c1 & c2).iter().next().unwrap())
            .collect();

        shared.iter().map(|c| Self::priority(*c)).sum()
    }

    pub fn part2(&self) -> usize {
        let badges: Vec<char> = self.input[..]
            .chunks(3)
            .map(|chunk| {
                if let [b1, b2, b3] = chunk {
                    *(&(&(&b1.0 | &b1.1) & &(&b2.0 | &b2.1)) & &(&b3.0 | &b3.1))
                        .iter()
                        .next()
                        .unwrap()
                } else {
                    panic!("Could not chunk")
                }
            })
            //.inspect(|c| println!("{}", c))
            .collect();
        badges.iter().map(|c| Self::priority(*c)).sum()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &'static str = "vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw";

    #[test]
    fn solve() {
        let parse = Day03::parse(SAMPLE);
        println!("{:?}", parse.input);
        assert_eq!(parse.part1(), 157);
        assert_eq!(parse.part2(), 70);
    }
}
