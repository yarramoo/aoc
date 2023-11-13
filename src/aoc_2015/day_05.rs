use std::collections::HashMap;

use crate::{IChallenge, Answer};

pub const DATA_PATH: &str = "src/aoc_2015/input/day_05";

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
    false
}

fn is_nice_2(s: &str) -> bool {
    // It contains a pair of any two letters that appears at least twice in the string without overlapping, 
    // like xyxy (xy) or aabcdefgaa (aa), but not like aaa (aa, but it overlaps).
    let mut pairs = HashMap::new();
    let mut found_pair = false;
    for (i, window) in s.as_bytes().windows(2).enumerate() {
        if let Some(j) = pairs.get(window) {
            if *j + 1 != i {
                found_pair = true;
                break;
            } 
        } else {
            pairs.insert(window, i);
        }
    }
    if !found_pair { return false; }

    // It contains at least one letter which repeats with exactly one letter between them, like xyx, 
    // abcdefeghi (efe), or even aaa.
    for window in s.as_bytes().windows(3) {
        if window[0] == window[2] {
            return true;
        }
    }
    false
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
        let count = self.input.lines()
            .filter(|l| is_nice_2(l))
            .count();
        Answer::Num(count as isize)
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

#[test]
fn test_nice_2() {
    let nice = &"qjhvhtzxzqqjkmpb";
    assert!(is_nice_2(nice));
    let nice = &"xxyxx";
    assert!(is_nice_2(nice));
}

#[test]
fn test_not_nice_2() {
    let not_nice = &"uurcxstgmygtbstg";
    assert!(!is_nice_2(not_nice));
    let not_nice = &"ieodomkazucvgmuy";
    assert!(!is_nice_2(not_nice));
}

#[test]
fn test_bad_case() {
    let not_nice = &"ueihvxviirnooomi";
    assert!(!is_nice_2(not_nice));
}