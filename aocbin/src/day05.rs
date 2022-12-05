use regex::Regex;
type Input = (Vec<Vec<char>>, Vec<Vec<usize>>);

fn parse(input: &str) -> Input {
    let (state, instructions) = input.split_once("\n\n").unwrap();

    let stacks_count = 1 + ((state.lines().next().unwrap().len() - 1) / 4);
    let mut stacks: Vec<Vec<char>> = vec![vec![]; stacks_count];
    state.lines()
         .rev()
         .skip(1)
         .map(|l| {
            l
                .chars()
                .skip(1)
                .step_by(4)
                .enumerate()
                .map(|c| {
                    match c {
                        (_i,  ' ') => (),
                        (i, crate_) => stacks[i].push(crate_),
                    };
                })
                .for_each(drop)
            })
            .for_each(drop);

    let re = Regex::new(r"move (\d+) from (\d+) to (\d+)").unwrap();
    let instructions: Vec<Vec<usize>> = 
        instructions
        .lines()
        .map(|l| {
            re.captures(l).unwrap().iter().skip(1).map(|c| c.unwrap().as_str().parse().unwrap()).collect()
        })
        .collect();

    (stacks, instructions)
}

fn part1(input: &Input) -> String {
    let mut state = input.0.clone();
    for i in &input.1[..] {
        if let [qty, from, to] = &i[..] {
            for _i in 0..*qty {
                let moved = state[from - 1].pop().unwrap();
                state[to - 1].push(moved)
            }
        } else {
            panic!();
        }
    }

    state.iter()
         .map(|s| s.last().unwrap().to_owned())
         .collect()
}

fn part2(input: &Input) -> String {
    let mut state = input.0.clone();
    for i in &input.1[..] {
        if let [qty, from, to] = &i[..] {
            let idx = state[from - 1].len() - qty;
            let mut moved = state[from - 1].split_off(idx);
            state[to - 1].append(&mut moved)
        } else {
            panic!();
        }
    }

    state.iter()
         .map(|s| s.last().unwrap().to_owned())
         .collect()
}

fn main() {
    let input = include_str!("../inputs/day05.txt");
    let parsed = parse(input);

    println!("{}", part1(&parsed));
    println!("{}", part2(&parsed));
}
