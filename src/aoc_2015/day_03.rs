use std::collections::HashSet;

use crate::{IChallenge, Answer};

pub const DATA_PATH: &str = "src/aoc_2015/input/day_03";

pub struct Challenge {
    input: String,
}

impl IChallenge for Challenge {
    fn parse(data: &str) -> Self {
        Challenge { input: data.into() }
    }

    fn solve_p1(&self) -> Answer {
        let mut unique_locations = HashSet::new();
        let mut location = Location::new(0, 0);
        for c in self.input.chars() {
            unique_locations.insert(location);
            location.move_direction(c);
        }
        Answer::Num(unique_locations.len() as isize)
    }

    fn solve_p2(&self) -> Answer {
        let mut unique_locations = HashSet::new();
        let mut location_a = Location::new(0, 0);
        let mut location_b = Location::new(0, 0);
        for (i, c) in self.input.chars().enumerate() {
            if i % 2 == 0 {
                unique_locations.insert(location_a);
                location_a.move_direction(c);
            } else {
                unique_locations.insert(location_b);
                location_b.move_direction(c);
            }
        }
        Answer::Num(unique_locations.len() as isize)
    }
}

#[derive(Clone, Copy, Hash, PartialEq, Eq)]
struct Location{ x: isize, y: isize }

impl Location {
    fn new(x: isize, y: isize) -> Self {
        Location { x, y }
    }
    fn move_direction(&mut self, dir: char) {
        match dir {
            '^' => self.y += 1,
            'v' => self.y -= 1,
            '>' => self.x += 1,
            '<' => self.x -= 1,
            _   => panic!("Unknown character")
        };
    }
}


