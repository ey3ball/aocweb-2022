#[cfg(feature = "wasm")]
use wasm_bindgen::prelude::*;

type Supplies = Vec<Vec<usize>>;

#[cfg_attr(feature = "wasm", wasm_bindgen)]
pub struct Day01 {
    supplies: Supplies,
}

#[cfg_attr(feature = "wasm", wasm_bindgen)]
impl Day01 {
    pub fn parse(input: &str) -> Day01 {
        Day01 {
            supplies: input
                .split("\n\n")
                .map(|inventory| inventory.lines().map(|l| l.parse().unwrap()).collect())
                .collect(),
        }
    }

    pub fn part1(&self) -> usize {
        self.supplies.iter().map(|s| s.iter().sum()).max().unwrap()
    }

    pub fn part2(&self) -> usize {
        let mut sum_supplies: Vec<usize> = self
            .supplies
            .iter()
            .map(|s| s.iter().sum())
            .collect::<Vec<usize>>();
        sum_supplies.sort();

        let end = sum_supplies.len();
        sum_supplies[end - 1] + sum_supplies[end - 2] + sum_supplies[end - 3]
    }
}
