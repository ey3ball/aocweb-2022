use aoclib::day05::Day05;

fn main() {
    let input = include_str!("../inputs/day05.txt");
    let day = Day05::parse(input);
    println!("{}", day.part1());
    println!("{}", day.part2());
}
