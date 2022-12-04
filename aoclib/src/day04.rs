type Section = std::ops::RangeInclusive<usize>;

pub struct Day04 {
    sections: Vec<(Section, Section)>,
}

fn contained(first: &Section, second: &Section) -> bool {
    (first.contains(second.start()) && first.contains(second.end()))
        || (second.contains(first.start()) && second.contains(first.end()))
}

fn overlap(first: &Section, second: &Section) -> bool {
    second.clone().any(|i| first.contains(&i))
}

impl Day04 {
    pub fn parse(input: &str) -> Day04 {
        Day04 {
            sections: input.lines()
             .map(|l| {
                let parsed: Vec<usize> = l.split(",").flat_map(
                     |e| e.split("-").map(|num| num.parse().unwrap())
                ).collect();
                ((parsed[0]..=parsed[1]),(parsed[2]..=parsed[3]))
             })
             .collect()
        }
    }

    pub fn part1(&self) -> usize {
        self.sections.iter()
                .filter(|(s1,s2)| contained(s1, s2))
                .count()
    }

    pub fn part2(&self) -> usize {
        self.sections.iter()
                .filter(|(s1,s2)| overlap(s1, s2))
                .count()
    }
}
