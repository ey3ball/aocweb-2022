pub mod day01;
pub mod day04;
pub mod day05;

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
