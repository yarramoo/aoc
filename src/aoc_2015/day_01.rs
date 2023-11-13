use crate::{IChallenge, Answer};

pub const DATA_PATH: &str = "src/aoc_2015/input/day_01";


fn solve_1(input: &str) -> isize {
    input
        .chars()
        .map(|c| {
            match c {
                '(' => 1,
                ')' => -1,
                _ => panic!("Unrecognised symbol"),
            }
        })
        .sum()
}

fn solve_2(input: &str) -> isize {
    let mut floor = 0;
    for (i, c) in input.chars().enumerate() {
        match c {
            '(' => floor += 1,
            ')' => floor -= 1,
            _   => panic!("Unknown character"),
        };
        if floor < 0 {
            return (i + 1) as isize;
        }
    }
    0
}

pub struct Challenge {
    input: String,
}

impl IChallenge for Challenge {
    fn parse(data: &str) -> Self {
        Challenge { input: data.to_string() }
    }

    fn solve_p1(&self) -> Answer {
        Answer::Num(solve_1(&self.input))
    }

    fn solve_p2(&self) -> Answer {
        Answer::Num(solve_2(&self.input))
    }
}