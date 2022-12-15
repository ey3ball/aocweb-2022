use regex::Regex;
use std::collections::HashSet;

type Input = String;
type Pos = (isize, isize);

pub struct Day15 {
    input: Vec<(Pos, Pos)>,
}

pub fn dst(first: &Pos, other: &Pos) -> isize {
    (other.0 - first.0).abs() + (other.1 - first.1).abs()
}

impl Day15 {
    pub fn parse(input: &str) -> Day15 {
        let re = Regex::new(r".*x=([\d-]*), y=([\d-]*).*x=([\d-]*), y=([\d-]*)").unwrap();

        let parsed: Vec<(Pos, Pos)> = input
            .lines()
            .map(|l| {
                let parsed = re.captures(l).unwrap();
                ((parsed[1].parse().unwrap(), parsed[2].parse().unwrap()),
                 (parsed[3].parse().unwrap(), parsed[4].parse().unwrap()))
            })
            .collect();

        Day15 {
            input: parsed
        }
    }

    pub fn part1(&self, target: isize) -> usize {
        self
            .input
            .iter()
            .map(|(sensor, beacon)| {
                let distance = dst(sensor, beacon);
                let dy = (target - sensor.1).abs();
                if dy <= distance {
                    ((sensor.0 - (distance - dy).abs()),(sensor.0 + (distance - dy).abs()))
                } else {
                    (0,0)
                }
            })
            .inspect(|dead| println!("{:?}", dead))
            .flat_map(|(xmin, xmax)| xmin..xmax)
            .collect::<HashSet<isize>>()
            .iter()
            .count()
    }

    pub fn part2(&self) -> usize {
        0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &'static str = "Sensor at x=2, y=18: closest beacon is at x=-2, y=15
Sensor at x=9, y=16: closest beacon is at x=10, y=16
Sensor at x=13, y=2: closest beacon is at x=15, y=3
Sensor at x=12, y=14: closest beacon is at x=10, y=16
Sensor at x=10, y=20: closest beacon is at x=10, y=16
Sensor at x=14, y=17: closest beacon is at x=10, y=16
Sensor at x=8, y=7: closest beacon is at x=2, y=10
Sensor at x=2, y=0: closest beacon is at x=2, y=10
Sensor at x=0, y=11: closest beacon is at x=2, y=10
Sensor at x=20, y=14: closest beacon is at x=25, y=17
Sensor at x=17, y=20: closest beacon is at x=21, y=22
Sensor at x=16, y=7: closest beacon is at x=15, y=3
Sensor at x=14, y=3: closest beacon is at x=15, y=3
Sensor at x=20, y=1: closest beacon is at x=15, y=3";

    #[test]
    fn solve() {
        let parse = Day15::parse(SAMPLE);
        assert_eq!(parse.part1(10), 0);
        assert_eq!(parse.part2(), 0);
    }
}
