use aoclib::day04::Day04;

fn main() {
    let input = include_str!("../inputs/day04.txt");
    let day = Day04::parse(input);
    println!("{}", day.part1());
    println!("{}", day.part2());
}
