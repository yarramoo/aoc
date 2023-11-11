use crate::{IChallenge, Answer};

pub const DATA_PATH: &str = &"src/aoc_2015/input/day_05";

fn is_nice(s: &str) -> bool {
    let forbidden = vec!["ab", "cd", "pq", "xy"];
    for f in forbidden {
        if s.contains(f) {
            return false;
        }
    }
    let mut vowels = 0;
    let mut found_double = false;
    let mut last_char = None;
    for c in s.chars() {
        if "aeiou".contains(c) { vowels += 1; }
        
        if let Some(last_char) = last_char {
            if last_char == c {
                found_double = true;
            }
        }
        last_char = Some(c);

        if found_double && vowels >= 3 {
            return true;
        }
    }
    return false;
}

pub struct Challenge {
    input: String,
}

impl IChallenge for Challenge {
    fn parse(data: &str) -> Self {
        Challenge { input: data.to_string() }
    }

    fn solve_p1(&self) -> Answer {
        let count = self.input.lines()
            .filter(|l| is_nice(l))
            .count();
        Answer::Num(count as isize)
    }

    fn solve_p2(&self) -> Answer {
        todo!();
    }
}

#[test]
fn test_nice() {
    let nice = &"ugknbfddgicrmopn";
    assert!(is_nice(nice));
    let nice = &"aaa";
    assert!(is_nice(nice));
}

#[test]
fn test_not_nice() {
    let not_nice = &"jchzalrnumimnmhp";
    assert!(!is_nice(not_nice));
    let not_nice = &"haegwjzuvuyypxyu";
    assert!(!is_nice(not_nice));
    let not_nice = &"dvszwmarrgswjxmb";
    assert!(!is_nice(not_nice));
}
