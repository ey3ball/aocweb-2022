use itertools::Itertools;
use std::collections::HashSet;

type Pos = (isize, isize);
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
        let walls: Cave = input
            .lines()
            .flat_map(|l| {
                l.split(" -> ")
                    .map(|coord| {
                        let (x, y) = coord.split_once(",").unwrap();
                        (x.parse::<isize>().unwrap(), y.parse::<isize>().unwrap())
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

    pub fn display(&self) {
        display(&self.input);
    }

    pub fn part1(&self) -> usize {
        let mut fill = self.input.clone();
        let ((xmin, _), (xmax, ymax)) = bounds(&fill);
        println!("x {:?} {:?} y {:?}", xmin, xmax, ymax);
        let mut i = 0;

        'outer: loop {
            let mut pos = (500, 0);
            i += 1;
            loop {
                if let Some(new_pos) = [(0, 1), (-1, 1), (1, 1)]
                    .iter()
                    .map(|d| (pos.0 + d.0, pos.1 + d.1))
                    .find(|p| !fill.contains(&p))
                {
                    pos = new_pos;

                    if !((xmin..=xmax).contains(&pos.0)) || pos.1 > ymax {
                        break 'outer;
                    }
                } else {
                    fill.insert(pos);
                    break;
                }
            }
        }
        i - 1
    }

    pub fn part2(&self) -> usize {
        let cave = &self.input;
        let (_, (_, ymax)) = bounds(&self.input);
        let ymax = ymax + 2;
        let mut sand: HashSet<isize> = HashSet::new();
        let (xmin, xmax) = (500, 500);
        sand.insert(500);
        let (_, count, _) = (1..ymax).fold((sand, 1, (xmin, xmax)), |(prev_sand, count, xs), y| {
            let xs = (xs.0 - 1, xs.1 + 1);
            let next_sand: HashSet<isize> = (xs.0..=xs.1)
                .filter(|x| {
                    (prev_sand.contains(&(x - 1))
                        || prev_sand.contains(&x)
                        || prev_sand.contains(&(x + 1)))
                        && !cave.contains(&(*x, y))
                })
                .collect();
            let count = count + next_sand.len();

            (next_sand, count, xs)
        });
        count
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
        assert_eq!(parse.part1(), 24);
        assert_eq!(parse.part2(), 93);
    }
}
