use crate::{IChallenge, Answer};

pub const DATA_PATH: &str = "src/aoc_2015/input/day_10";

fn run_length_encode(sequence: &[u32]) -> Vec<(u32, u32)> {
    let mut result = Vec::new();
    let mut last = sequence[0];
    let mut last_occurances = 0;
    for &n in sequence {
        if n == last {
            last_occurances += 1;
        } else {
            result.push((last, last_occurances));
            last = n;
            last_occurances = 1;
        }
    }
    result.push((last, last_occurances));
    result
}

fn look_and_say(sequence: &[u32]) -> Vec<u32> {
    let rle = run_length_encode(sequence);
    let mut result = Vec::new();
    for entry in rle {
        result.push(entry.1);
        result.push(entry.0);
    }
    result
}


pub struct Challenge {
    look_and_say_sequence: Vec<u32>,
}

impl IChallenge for Challenge {
    fn parse(input: &str) -> Self {
        let mut sequence = Vec::new();
        for c in input.chars() {
            let n = c.to_digit(10).unwrap();
            sequence.push(n);
        }
        Challenge { look_and_say_sequence: sequence }
    }

    fn solve_p1(&self) -> Answer {
        let mut result = self.look_and_say_sequence.clone();
        for _ in 0..40 {
            result = look_and_say(&result[..]);
        }
        Answer::Num(result.len() as isize)
    }

    fn solve_p2(&self) -> Answer {
        let mut result = self.look_and_say_sequence.clone();
        for _ in 0..50 {
            result = look_and_say(&result[..]);
        }
        Answer::Num(result.len() as isize)
    }
}

#[test]
fn test_rle() {
    let sequence = vec![0, 1, 1, 2, 2, 2, 3, 3, 3, 3, 4, 3, 0];
    let rle = run_length_encode(&sequence[..]);
    assert_eq!(
        rle,
        vec![(0,1), (1,2), (2,3), (3,4), (4,1), (3,1), (0,1)]
    );
}

#[test]
fn test_look_and_say() {
    let input = vec![1];
    let output = look_and_say(&input[..]);
    assert_eq!(output, vec![1,1]);

    let input = vec![1,1];
    let output = look_and_say(&input[..]);
    assert_eq!(output, vec![2,1]);

    let input = vec![1,1,1,2,2,1];
    let output = look_and_say(&input[..]);
    assert_eq!(output, vec![3,1,2,2,1,1]);
}