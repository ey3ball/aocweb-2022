use std::collections::HashSet;
use std::collections::VecDeque;
type Input = Vec<char>;

pub struct Day06 {
    input: Input,
}

impl Day06 {
    pub fn parse(input: &str) -> Day06 {
        Day06 {
            input: input.chars().collect(),
        }
    }

    pub fn find_marker(&self, size: usize) -> usize {
        let mut header: VecDeque<char> = VecDeque::new();
        for (i, c) in self.input.iter().enumerate() {
            if header.len() < size {
                header.push_back(*c);
            } else {
                header.pop_front();
                header.push_back(*c);
                let symbols: HashSet<char> = header.iter().copied().collect();
                if symbols.len() == size {
                    return i + 1;
                }
            }
        }
        panic!("not found")
    }

    pub fn part1(&self) -> usize {
        self.find_marker(4)
    }

    pub fn part2(&self) -> usize {
        self.find_marker(14)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &str = "mjqjpqmgbljsphdztnvjfqwrcgsmlb";
    const SAMPLE_2: &str = "bvwbjplbgvbhsrlpgdmjqwftvncz";
    const SAMPLE_3: &str = "nppdvjthqldpwncqszvftbrmjlhg";
    const SAMPLE_4: &str = "nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg";
    const SAMPLE_5: &str = "zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw";

    #[test]
    fn part1() {
        let parse = Day06::parse(SAMPLE);
        assert_eq!(parse.part1(), 7);
    }

    #[test]
    fn part2() {
        let parse = Day06::parse(SAMPLE);
        let parse_2 = Day06::parse(SAMPLE_2);
        let parse_3 = Day06::parse(SAMPLE_3);
        let parse_4 = Day06::parse(SAMPLE_4);
        let parse_5 = Day06::parse(SAMPLE_5);
        assert_eq!(parse.part2(), 19);
        assert_eq!(parse_2.part2(), 23);
        assert_eq!(parse_3.part2(), 23);
        assert_eq!(parse_4.part2(), 29);
        assert_eq!(parse_5.part2(), 26);
    }
}
