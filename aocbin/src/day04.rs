
type Section = std::ops::RangeInclusive<usize>;
type Sections = Vec<(Section, Section)>;

fn parse(input: &str) -> Sections {
    input.lines()
         .map(|l| {
            let parsed: Vec<usize> = l.split(",").flat_map(
                 |e| e.split("-").map(|num| num.parse().unwrap())
            ).collect();
            ((parsed[0]..=parsed[1]),(parsed[2]..=parsed[3]))
         })
         .collect()
}

fn part1(sections: &Sections) -> usize {
    sections.iter()
            .filter(|(s1,s2)| contained(s1, s2))
            .count()
}

fn part2(sections: &Sections) -> usize {
    sections.iter()
            .filter(|(s1,s2)| overlap(s1, s2))
            .count()
}

fn contained(first: &Section, second: &Section) -> bool {
    (first.contains(second.start()) && first.contains(second.end()))
        || (second.contains(first.start()) && second.contains(first.end()))
}

fn overlap(first: &Section, second: &Section) -> bool {
    second.clone().any(|i| first.contains(&i))
}


fn main() {
    let input = include_str!("../inputs/day04.txt");
    let parsed = parse(input);
    println!("{}", part1(&parsed));
    println!("{}", part2(&parsed));
}
