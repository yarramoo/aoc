use std::{collections::HashMap, panic::UnwindSafe};
use fancy_regex::Regex;

use crate::{IChallenge, Answer};

pub const DATA_PATH: &str = "src/aoc_2015/input/day_05";

fn is_nice(s: &str) -> bool {
    let rule_1 = Regex::new(r"(.*[aeiou]){3}.*").unwrap();
    let rule_2 = Regex::new(r"(.)\1").unwrap();
    let rule_3 = Regex::new(r"ab|cd|pq|xy").unwrap();
    rule_1.is_match(s).unwrap() && rule_2.is_match(s).unwrap() && !rule_3.is_match(s).unwrap()
}

fn is_nice_2(s: &str) -> bool {
    let rule_1 = Regex::new(r"(..).*\1").unwrap();
    let rule_2 = Regex::new(r"(.).\1").unwrap();
    rule_1.is_match(s).unwrap() && rule_2.is_match(s).unwrap()
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