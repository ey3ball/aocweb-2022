use aoclib::day01;

fn main() {
    let input = include_str!("../inputs/day01.txt");
    let day = day01::Day01::parse(input);
    println!("{}", day.part1());
    println!("{}", day.part2());
}
