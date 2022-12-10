use std::collections::HashSet;
type Input = Vec<(isize, isize)>;
type Pos = (isize, isize);

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
                (0..l[2..].parse().unwrap()).map(|_| match l.as_bytes() {
                    [b'R', ..] => (1, 0),
                    [b'L', ..] => (-1, 0),
                    [b'U', ..] => (0, -1),
                    [b'D', ..] => (0, 1),
                    _ => panic!(),
                })
            })
            .collect();

        Day09 { input: parsed }
    }

    pub fn knot_move(new_head: Pos, head: Pos, tail: Pos) -> Pos {
        let move_: Pos = (new_head.0 - head.0, new_head.1 - head.1);
        if (new_head.0 - tail.0).abs() == 2 && (new_head.1 - tail.1).abs() == 2 {
            (tail.0 + move_.0, tail.1 + move_.1)
        } else if (new_head.0 - tail.0).abs() == 2 {
            (tail.0 + move_.0, new_head.1)
        } else if (new_head.1 - tail.1).abs() == 2 {
            (new_head.0, tail.1 + move_.1)
        } else {
            tail
        }
    }

    pub fn part1(&self) -> usize {
        let mut visited: HashSet<(isize, isize)> = HashSet::new();
        self.input
            .iter()
            .fold(((0, 0), (0, 0)), |(head, tail), move_| {
                let new_head = (head.0 + move_.0, head.1 + move_.1);
                let new_tail = Self::knot_move(new_head, head, tail);
                visited.insert(new_tail);
                (new_head, new_tail)
            });
        visited.len()
    }

    pub fn part2(&self) -> usize {
        let mut visited: HashSet<(isize, isize)> = HashSet::new();
        self.input.iter().fold(vec![(0, 0); 10], |hts, move_| {
            let mut new_head = (hts[0].0 + move_.0, hts[0].1 + move_.1);
            let mut new_hts = vec![];
            new_hts.push(new_head);
            for head_tail in hts.windows(2) {
                let new_tail = Self::knot_move(new_head, head_tail[0], head_tail[1]);
                new_hts.push(new_tail);
                new_head = new_tail;
            }
            visited.insert(*new_hts.last().unwrap());
            new_hts
        });
        visited.len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &str = "R 4
U 4
L 3
D 1
R 4
D 1
L 5
R 2";

    const SAMPLE_2: &str = "R 5
U 8
L 8
D 3
R 17
D 10
L 25
U 20";

    #[test]
    fn solve_p1() {
        let parse = Day09::parse(SAMPLE);
        assert_eq!(parse.part1(), 13);
    }

    #[test]
    fn solve_p2() {
        let parse = Day09::parse(SAMPLE);
        assert_eq!(parse.part2(), 1);
    }

    #[test]
    fn solve_p2b() {
        let parse = Day09::parse(SAMPLE_2);
        assert_eq!(parse.part2(), 36);
    }
}
