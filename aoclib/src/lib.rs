pub mod day01;
pub mod day02;
pub mod day03;
pub mod day04;
pub mod day05;
pub mod day06;
pub mod day08;
pub mod day09;
pub mod day10;

pub fn add(left: usize, right: usize) -> usize {
    left + right
}

pub fn hello() -> String {
    " from aoclib ".to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
