use md5;

use crate::{IChallenge, Answer};

pub const DATA_PATH: &str = &"src/aoc_2015/input/day_04";

pub struct Challenge {
    input: String,
}

impl IChallenge for Challenge {
    fn parse(data: &str) -> Self {
        Challenge { input: data.to_string() }
    }

    fn solve_p1(&self) -> Answer {
        for i in 0.. {
            let attempt = format!("{}{}", self.input, i);
            let hash = format!("{:x}", md5::compute(attempt.as_bytes()));
            let hash = hash.as_bytes();
            if hash[0] == '0' as u8 &&
                hash[1] == '0' as u8 &&
                hash[2] == '0' as u8 &&
                hash[3] == '0' as u8 &&
                hash[4] == '0' as u8 
            {
                return Answer::Num(i);
            }
        }
        Answer::Num(0)
    }

    fn solve_p2(&self) -> Answer {

        for i in 0.. {
            let attempt = format!("{}{}", self.input, i);
            let hash = format!("{:x}", md5::compute(attempt.as_bytes()));
            let hash = hash.as_bytes();
            if hash[0] == '0' as u8 &&
                hash[1] == '0' as u8 &&
                hash[2] == '0' as u8 &&
                hash[3] == '0' as u8 &&
                hash[4] == '0' as u8 &&
                hash[5] == '0' as u8 
            {
                return Answer::Num(i);
            }
        }
        Answer::Num(0)
    }
}