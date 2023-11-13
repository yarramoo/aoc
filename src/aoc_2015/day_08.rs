use crate::{IChallenge, Answer};

pub const DATA_PATH: &str = "src/aoc_2015/input/day_08";

fn count_chars(s: &str) -> usize {
    let mut total = 0;
    let mut chars = s.chars();
    while let Some(c) = chars.next() {
        if c == '\\' {
            if chars.next().unwrap() == 'x' {
                let _ = chars.next();
                let _ = chars.next();
            }
        }
        total += 1;
    }
    total - 2
}


pub struct Challenge {
    data: String
}

impl IChallenge for Challenge {
    fn parse(input: &str) -> Self {
        Challenge { data: input.to_string() } 
    }

    fn solve_p1(&self) -> Answer {
        let mut total = 0;
        for l in self.data.lines() {
            total += l.chars().count() - count_chars(l);
        }
        Answer::Num(total as isize)
    }

    fn solve_p2(&self) -> Answer {
        let mut added = 0;
        for l in self.data.lines() {
            for c in l.chars() {
                if c == '\\' || c == '\"' {
                    added += 1;
                }
            }
            added += 2;
        }
        Answer::Num(added)
    }
}


#[test]
fn test_count_chars() {

}