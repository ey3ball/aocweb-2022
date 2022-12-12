use pathfinding::matrix::Matrix;

type Input = Matrix<char>;

pub struct Day12 {
    input: Input,
    start: (usize, usize),
    end: (usize, usize),
}

impl Day12 {
    pub fn parse(input: &str) -> Day12 {
        let mut matrix = Matrix::from_rows(input.lines().map(|l| l.chars())).unwrap();

        let start = matrix.indices().find(|&pos| matrix[pos] == 'S').unwrap();
        let end = matrix.indices().find(|&pos| matrix[pos] == 'E').unwrap();
        matrix[start] = 'a';
        matrix[end] = 'z';
        Day12 {
            input: matrix,
            start,
            end,
        }
    }

    pub fn part1(&self) -> usize {
        println!("{:?}", self.start);
        println!("{:?}", self.end);
        let result = pathfinding::directed::bfs::bfs(
            &self.start,
            |&p| {
                let val = self.input[p];
                self.input
                    .neighbours(p, false)
                    // .inspect(|p| println!("t {:?}", p))
                    // .inspect(move |&n| println!("t {:?} {} {} {}", n, ('a' as usize),(val as usize + 1), self.input[n] as usize))
                    .filter(move |&n| {
                        (('a' as usize)..=(val as usize + 1)).contains(&(self.input[n] as usize))
                    })
            },
            |&p| p == self.end,
        );
        result.unwrap().len()
    }

    pub fn part2(&self) -> usize {
        0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &'static str = "Sabqponm
abcryxxl
accszExk
acctuvwj
abdefghi";

    #[test]
    fn solve() {
        let parse = Day12::parse(SAMPLE);
        println!("{:#?}", parse.input);
        assert_eq!(parse.part1(), 32);
        assert_eq!(parse.part2(), 0);
    }
}
