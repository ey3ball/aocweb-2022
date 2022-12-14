use itertools::Itertools;
use std::collections::HashSet;

type Pos = (usize, usize);
type Cave = HashSet<Pos>;

pub struct Day14 {
    input: Cave,
}

pub fn bounds(cave: &Cave) -> (Pos, Pos) {
    let xmin = *cave.iter().map(|(x, _)| x).min().unwrap();
    let xmax = *cave.iter().map(|(x, _)| x).max().unwrap();
    let ymin = *cave.iter().map(|(_, y)| y).min().unwrap();
    let ymax = *cave.iter().map(|(_, y)| y).max().unwrap();

    ((xmin, ymin), (xmax, ymax))
}

pub fn display(cave: &Cave) {
    let ((xmin, ymin), (xmax, ymax)) = bounds(cave);

    for y in ymin..=ymax {
        for x in xmin..=xmax {
            if cave.contains(&(x, y)) {
                print!("▓");
            } else {
                print!(" ");
            }
        }
        print!("\n");
    }
}

impl Day14 {
    pub fn parse(input: &str) -> Day14 {
        let walls: HashSet<(usize, usize)> = input
            .lines()
            .flat_map(|l| {
                l.split(" -> ")
                    .map(|coord| {
                        let (x, y) = coord.split_once(",").unwrap();
                        (x.parse::<usize>().unwrap(), y.parse::<usize>().unwrap())
                    })
                    .tuple_windows()
                    .flat_map(|((x1, y1), (x2, y2))| {
                        (x1.min(x2)..=x1.max(x2))
                            .flat_map(move |x| (y1.min(y2)..=y1.max(y2)).map(move |y| (x, y)))
                    })
            })
            .collect();

        Day14 { input: walls }
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

    const SAMPLE: &'static str = "498,4 -> 498,6 -> 496,6
503,4 -> 502,4 -> 502,9 -> 494,9
";

    #[test]
    fn solve() {
        let parse = Day14::parse(SAMPLE);
        display(&parse.input);
        assert_eq!(parse.part1(), 0);
        assert_eq!(parse.part2(), 0);
    }
}
