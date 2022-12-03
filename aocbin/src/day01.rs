use aoclib::day01;

fn main() {
    let input = include_str!("../inputs/day01.txt");
    let parsed = day01::parse(input);
    println!("{}", day01::part1(&parsed));
    println!("{}", day01::part2(&parsed));
}
