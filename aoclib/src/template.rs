type Input = String;

pub struct Day {
    input: Input,
}

impl Day {
    pub fn parse(input: &str) -> Day {
        Day {
            input: input.to_string()
        }
    }

    pub fn part1(&self) -> usize {
        0
    }

    pub fn part2(&self) -> usize {
        0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &'static str = "";

    #[test]
    fn solve() {
        let parse = Day::parse(SAMPLE);
        assert_eq!(parse.part1(), 0);
        assert_eq!(parse.part2(), 0);
    }
}
