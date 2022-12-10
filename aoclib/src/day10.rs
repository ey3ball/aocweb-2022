use pathfinding::grid::Grid;

#[derive(Debug)]
enum Inst {
    AddX(isize),
    Noop,
}

type Input = Vec<Inst>;

#[derive(Debug)]
pub struct Day10 {
    input: Input,
}

impl Day10 {
    pub fn parse(input: &str) -> Day10 {
        let mut parsed: Vec<Inst> = input
            .lines()
            .flat_map(|l| {
                if l.starts_with("noop") {
                    vec![Inst::Noop]
                } else if l.starts_with("addx") {
                    vec![
                        Inst::Noop,
                        Inst::AddX(l.split_once(' ').unwrap().1.parse().unwrap()),
                    ]
                } else {
                    panic!()
                }
            })
            .collect();
        parsed.push(Inst::Noop);

        Day10 { input: parsed }
    }

    pub fn part1(&self) -> isize {
        self.input
            .iter()
            .scan(1, |x, inst| match inst {
                Inst::Noop => Some(*x),
                Inst::AddX(v) => {
                    *x += v;
                    Some(*x - v)
                }
            })
            .enumerate()
            //            .inspect(|(cnt, x)| println!("{}: {}", cnt + 1, x))
            .take(220)
            .skip(19)
            .step_by(40)
            //            .inspect(|(cnt, x)| println!("{}: {}", cnt + 1, x))
            .map(|(cnt, x)| (cnt + 1) as isize * x)
            .sum()
    }

    pub fn part2(&self) -> String {
        const SCREEN_WIDTH: usize = 40;
        let drawn = self
            .input
            .iter()
            .scan(1, |x, inst| match inst {
                Inst::Noop => Some(*x),
                Inst::AddX(v) => {
                    *x += v;
                    Some(*x - v)
                }
            })
            .enumerate()
            .filter_map(|(cycle, x)| {
                if (x - 1..=x + 1).contains(&((cycle % SCREEN_WIDTH) as isize)) {
                    Some((cycle % SCREEN_WIDTH, cycle / SCREEN_WIDTH))
                } else {
                    None
                }
            })
            .collect::<Grid>();
        format!("{:?}", drawn)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &str = "noop
addx 3
addx -5";
    const SAMPLE_2: &str = "addx 15
addx -11
addx 6
addx -3
addx 5
addx -1
addx -8
addx 13
addx 4
noop
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx -35
addx 1
addx 24
addx -19
addx 1
addx 16
addx -11
noop
noop
addx 21
addx -15
noop
noop
addx -3
addx 9
addx 1
addx -3
addx 8
addx 1
addx 5
noop
noop
noop
noop
noop
addx -36
noop
addx 1
addx 7
noop
noop
noop
addx 2
addx 6
noop
noop
noop
noop
noop
addx 1
noop
noop
addx 7
addx 1
noop
addx -13
addx 13
addx 7
noop
addx 1
addx -33
noop
noop
noop
addx 2
noop
noop
noop
addx 8
noop
addx -1
addx 2
addx 1
noop
addx 17
addx -9
addx 1
addx 1
addx -3
addx 11
noop
noop
addx 1
noop
addx 1
noop
noop
addx -13
addx -19
addx 1
addx 3
addx 26
addx -30
addx 12
addx -1
addx 3
addx 1
noop
noop
noop
addx -9
addx 18
addx 1
addx 2
noop
noop
addx 9
noop
noop
noop
addx -1
addx 2
addx -37
addx 1
addx 3
noop
addx 15
addx -21
addx 22
addx -6
addx 1
noop
addx 2
addx 1
noop
addx -10
noop
noop
addx 20
addx 1
addx 2
addx 2
addx -6
addx -11
noop
noop
noop";

    const RESULT: &str = "##..##..##..##..##..##..##..##..##..##..
###...###...###...###...###...###...###.
####....####....####....####....####....
#####.....#####.....#####.....#####.....
######......######......######......####
#######.......#######.......#######.....";
    #[test]
    fn solve() {
        let parse = Day10::parse(SAMPLE);
        assert_eq!(parse.input.len(), 6);
    }

    #[test]
    fn solve_2() {
        let parse = Day10::parse(SAMPLE_2);
        assert_eq!(parse.part1(), 13140);
        assert_eq!(parse.part2(), RESULT);
    }
}
