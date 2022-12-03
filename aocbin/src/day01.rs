type Supplies = Vec<Vec<usize>>;

fn parse(input: &str) -> Supplies {
    input
        .split("\n\n")
        .map(|inventory| inventory.lines().map(|l| l.parse().unwrap()).collect())
        .collect()
}

fn part1(supplies: &Supplies) -> usize {
    supplies.iter().map(|s| s.iter().sum()).max().unwrap()
}

fn part2(supplies: &Supplies) -> usize {
    let mut sum_supplies: Vec<usize> = supplies
        .iter()
        .map(|s| s.iter().sum())
        .collect::<Vec<usize>>();
    sum_supplies.sort();

    let end = sum_supplies.len();
    return sum_supplies[end - 1] + sum_supplies[end - 2] + sum_supplies[end - 3];
}

fn main() {
    let input = include_str!("../inputs/day01.txt");
    let parsed = parse(input);
    println!("{}", part1(&parsed));
    println!("{}", part2(&parsed));
}
