use crate::{IChallenge, Answer};

pub const DATA_PATH: &str = &"src/aoc_2015/input/day_02";

pub struct Challenge {
    presents: Vec<PresentDims>,
}

impl IChallenge for Challenge {
    fn parse(data: &str) -> Challenge {
        let presents = data
            .lines()
            .map(|l| PresentDims::from(l))
            .collect();
        Challenge { presents }
    }

    fn solve_p1(&self) -> Answer {
        Answer::Num(solve_1(&self.presents) as isize)
    }

    fn solve_p2(&self) -> Answer {
        Answer::Num(solve_2(&self.presents) as isize)
    }
}


struct PresentDims(usize, usize, usize);

impl PresentDims {
    fn from_sides(a: usize, b: usize, c: usize) -> Self {
        let mut sides = [a, b, c];
        sides.sort();
        PresentDims(sides[0], sides[1], sides[2])
    }

    fn wrapping_paper(&self) -> usize {
        2 * self.0 * self.1 +
            2 * self.0 * self.2 +
            2 * self.1 * self.2 +
            self.0 * self.1
    }

    fn ribbon(&self) -> usize {
        2 * self.0 + 2 * self.1 + self.0 * self.1 * self.2
    }
}

impl From<&str> for PresentDims {
    fn from(s: &str) -> Self {
        let mut dims = s.split('x');
        let a = dims.next().unwrap().parse().unwrap();
        let b = dims.next().unwrap().parse().unwrap();
        let c = dims.next().unwrap().parse().unwrap();
        PresentDims::from_sides(a, b, c)
    }
}

fn solve_1(presents: &Vec<PresentDims>) -> usize {
    presents
        .iter()
        .map(|p| p.wrapping_paper())
        .sum()
}

fn solve_2(presents: &Vec<PresentDims>) -> usize {
    presents
        .iter()
        .map(|p| p.ribbon())
        .sum()
}

