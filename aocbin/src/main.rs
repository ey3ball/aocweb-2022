use aoclib::day01::Day01 as Day;

fn main() {
    let input = include_str!("../inputs/day01.txt");
    let day = Day::parse(input);
    println!("{}", day.part1());
    println!("{}", day.part2());
}
