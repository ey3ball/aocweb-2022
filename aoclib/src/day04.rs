type Section = std::ops::RangeInclusive<usize>;

pub struct Day04 {
    sections: Vec<(Section, Section)>,
}

trait ExtendedRange {
    fn contained(&self, other: &Self) -> bool;
    fn overlaps(&self, other: &Self) -> bool;
}

impl ExtendedRange for Section {
    fn contained(&self, other: &Self) -> bool {
        other.contains(&self.start()) && other.contains(&self.end())
    }
    fn overlaps(&self, other: &Self) -> bool {
        self.clone().any(|i| other.contains(&i))
    }
}

impl Day04 {
    pub fn parse(input: &str) -> Day04 {
        Day04 {
            sections: input
                .lines()
                .map(|l| {
                    let parsed: Vec<usize> = l
                        .split(",")
                        .flat_map(|e| e.split("-").map(|num| num.parse().unwrap()))
                        .collect();
                    ((parsed[0]..=parsed[1]), (parsed[2]..=parsed[3]))
                })
                .collect(),
        }
    }

    pub fn part1(&self) -> usize {
        self.sections
            .iter()
            .filter(|(s1, s2)| s1.contained(s2) || s2.contained(s1))
            .count()
    }

    pub fn part2(&self) -> usize {
        self.sections
            .iter()
            .filter(|(s1, s2)| s1.overlaps(s2))
            .count()
    }
}
