use pathfinding::matrix::Matrix;
use std::collections::HashSet;
type Input = Matrix<u8>;

pub struct Day08 {
    input: Input,
}

impl Day08 {
    pub fn parse(input: &str) -> Day08 {
        let data: Vec<u8> = input
            .lines()
            .flat_map(|l| l.chars().map(|c| c.to_digit(10).unwrap() as u8))
            .collect();
        Day08 {
            input: Matrix::square_from_vec(data).unwrap()
        }
    }

    pub fn left(matrix: &Input) -> Vec<Vec<(usize, usize)>> {
        (0..matrix.rows)
            .map(|y| (0..matrix.columns).map(move |x| (x,y)).collect())
            .collect()
    }

    pub fn right(matrix: &Input) -> Vec<Vec<(usize, usize)>> {
        (0..matrix.rows)
            .map(|y| (0..matrix.columns).rev().map(move |x| (x,y)).collect())
            .collect()
    }

    pub fn top(matrix: &Input) -> Vec<Vec<(usize, usize)>> {
        (0..matrix.columns)
            .map(|x| (0..matrix.rows).map(move |y| (x,y)).collect())
            .collect()
    }

    pub fn bottom(matrix: &Input) -> Vec<Vec<(usize, usize)>> {
        (0..matrix.columns)
            .map(|x| (0..matrix.rows).rev().map(move |y| (x,y)).collect())
            .collect()
    }

    pub fn observe(tracker: &mut HashSet<usize>, matrix: &Input) {
        for view in [Self::left(&matrix), Self::right(&matrix), Self::top(&matrix), Self::bottom(&matrix)] {
            view
                .iter()
                .for_each(|dir| {
                    dir
                        .iter()
                        .fold(None, |max, &pos| if max.is_none() || matrix[pos] > max.unwrap() {
                            tracker.insert(matrix.idx(pos));
                            Some(matrix[pos])
                        } else {
                            max
                        });
                });
        }
    }

    pub fn part1(&self) -> usize {
        let mut matrix = self.input.clone();
        let mut visible: HashSet<usize> = HashSet::new();
        Self::observe(&mut visible, &mut matrix);
        visible.iter().count()
    }

    pub fn part2(&self) -> usize {
        0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &'static str = "30373
25512
65332
33549
35390";

    #[test]
    fn test_part1() {
        let parse = Day08::parse(SAMPLE);
        assert_eq!(parse.part1(), 21);
    }
}
