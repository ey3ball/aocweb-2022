use std::collections::HashSet;
type Input = Vec<(isize, isize)>;

pub struct Day09 {
    input: Input,
}

impl Day09 {
    pub fn opp(dir: (isize, isize)) -> (isize, isize) {
        (-dir.0, -dir.1)
    }

    pub fn parse(input: &str) -> Day09 {
        let parsed: Input = input
            .lines()
            .flat_map(|l| {
                (0..l[2..].parse().unwrap()).map(|_|
                    match l.as_bytes() {
                        [b'R', ..] => (1, 0),
                        [b'L', ..] => (-1, 0),
                        [b'U', ..] => (0, -1),
                        [b'D', ..] => (0, 1),
                        _ => panic!()
                    })
            })
            .collect();

        Day09 {
            input: parsed
        }
    }

    pub fn part1(&self) -> usize {
        let mut visited: HashSet<(isize, isize)> = HashSet::new();
        self.input
            .iter()
            .fold(((0,0),(0,0)), |(head, tail), move_| {
                let new_head = (head.0 + move_.0, head.1 + move_.1);
                let new_tail = if head == tail || new_head == tail {
                    tail
                } else if (new_head.0 - tail.0).abs() == 2 {
                    (tail.0 + move_.0, new_head.1)
                } else if (new_head.1 - tail.1).abs() == 2 {
                    (new_head.0, tail.1 + move_.1)
                } else {
                    tail
                };
                visited.insert(new_tail);
                (new_head, new_tail)
            });
        visited.iter().count()
    }

    pub fn part2(&self) -> usize {
        0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &'static str = "R 4
U 4
L 3
D 1
R 4
D 1
L 5
R 2";

    #[test]
    fn solve() {
        let parse = Day09::parse(SAMPLE);
        assert_eq!(parse.part1(), 13);
        assert_eq!(parse.part2(), 0);
    }
}
