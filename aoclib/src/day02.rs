type Input = Vec<(char, char)>;

pub struct Day02 {
    input: Input,
}

impl Day02 {
    pub fn parse(input: &str) -> Day02 {
        Day02 {
            input: input
                .lines()
                .map(|l| (l.chars().nth(0).unwrap(), l.chars().nth(2).unwrap()))
                .collect(),
        }
    }

    pub fn shape_score(c: char) -> usize {
        match c {
            'A' | 'X' => 1,
            'B' | 'Y' => 2,
            'C' | 'Z' => 3,
            _ => panic!(),
        }
    }

    pub fn wanted_outcome(c: char) -> usize {
        match c {
            'X' => 0,
            'Y' => 3,
            'Z' => 6,
            _ => panic!(),
        }
    }

    pub fn outcome(c1: char, c2: char) -> usize {
        let s1 = Self::shape_score(c1);
        let s2 = Self::shape_score(c2);
        match (s1, s2) {
            (_, _) if s1 == s2 => 3,
            (1, 3) => 0,
            (2, 1) => 0,
            (3, 2) => 0,
            _ => 6,
        }
    }

    pub fn part1(&self) -> usize {
        self.input
            .iter()
            .map(|&(c1, c2)| Self::outcome(c1, c2) + Self::shape_score(c2))
            .sum()
    }

    pub fn part2(&self) -> usize {
        self.input
            .iter()
            .map(|&(c1, c2)| {
                let wanted = Self::wanted_outcome(c2);
                let mut selected = ' ';
                for c in ['A', 'B', 'C'] {
                    if Self::outcome(c1, c) == wanted {
                        selected = c
                    }
                }

                Self::outcome(c1, selected) + Self::shape_score(selected)
            })
            .sum()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &'static str = "A Y
B X
C Z";

    #[test]
    fn solve() {
        let parse = Day02::parse(SAMPLE);
        assert_eq!(parse.part1(), 15);
        assert_eq!(parse.part2(), 12);
    }
}
