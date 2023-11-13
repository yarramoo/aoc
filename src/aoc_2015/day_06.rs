use std::str::FromStr;

use crate::{IChallenge, Answer};

pub const DATA_PATH: &str = "src/aoc_2015/input/day_06";

struct Coordinate {
    x: usize,
    y: usize,
}

impl FromStr for Coordinate {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut values = s.split(',');
        let x: usize = values.next().unwrap().parse().unwrap();
        let y: usize = values.next().unwrap().parse().unwrap();
        Ok(Coordinate { x, y })
    }
}

enum Instruction {
    On, Off, Toggle,
}

struct Grid {
    lights: [[bool; 1000]; 1000],
}

impl Grid {
    fn new() -> Self {
        Grid { lights: [[false; 1000]; 1000] }
    }

    fn apply_instruction(&mut self, instr: &Instruction, a: &Coordinate, b: &Coordinate) {
        for y in a.y..=b.y {
            for x in a.x..=b.x {
                match instr {
                    Instruction::On => self.lights[y][x] = true,
                    Instruction::Off => self.lights[y][x] = false,
                    Instruction::Toggle => self.lights[y][x] = !self.lights[y][x],
                }
            }
        }
    }

    fn count_on_lights(&self) -> usize {
        let mut total = 0;
        for y in 0..1000 {
            for x in 0..1000 {
                if self.lights[y][x] {
                    total += 1;
                }
            }
        }
        total
    }
}

struct BrightnessGrid {
    lights: [[u8; 1000]; 1000],
}

impl BrightnessGrid {
    fn new() -> Self {
        Self { lights: [[0; 1000]; 1000] }
    }

    fn apply_instruction(&mut self, instr: &Instruction, a: &Coordinate, b: &Coordinate) {
        for y in a.y..=b.y {
            for x in a.x..=b.x {
                match instr {
                    Instruction::On => self.lights[y][x] += 1,
                    Instruction::Off => {
                        self.lights[y][x] = if self.lights[y][x] == 0 {0} else {self.lights[y][x] - 1}
                    },
                    Instruction::Toggle => self.lights[y][x] += 2,
                }
            }
        }
    }

    fn count_on_lights(&self) -> usize {
        let mut total: usize = 0;
        for y in 0..1000 {
            for x in 0..1000 {
                total += self.lights[y][x] as usize;
            }
        }
        total
    }
}

pub struct Challenge {
    data: Vec<(Instruction, Coordinate, Coordinate)>,
}

impl IChallenge for Challenge {
    fn parse(data: &str) -> Self {
        let mut instrs = Vec::new();
        for l in data.lines() {
            let (instr, a, b) = 
            if l.starts_with("turn off") 
            {
                let mut w = l.split_ascii_whitespace();
                let a = w.nth(2).unwrap();
                let b = w.nth(1).unwrap();
                (Instruction::Off, a, b)
            } 
            else if l.starts_with("turn on") 
            {
                let mut w = l.split_ascii_whitespace();
                let a = w.nth(2).unwrap();
                let b = w.nth(1).unwrap();
                (Instruction::On, a, b)
            } 
            else
            {
                let mut w = l.split_ascii_whitespace();
                let a = w.nth(1).unwrap();
                let b = w.nth(1).unwrap();
                (Instruction::Toggle, a, b)
            };
            let a: Coordinate = a.parse().unwrap();
            let b: Coordinate = b.parse().unwrap();
            instrs.push((instr, a, b));
        }
        Challenge { data: instrs }
    }

    fn solve_p1(&self) -> Answer {
        let mut grid = Grid::new();
        for (instr, a, b) in self.data.iter() {
            grid.apply_instruction(instr, a, b);
        }
        Answer::Num(grid.count_on_lights() as isize)
    }

    fn solve_p2(&self) -> Answer {
        let mut grid = BrightnessGrid::new();
        for (instr, a, b) in self.data.iter() {
            grid.apply_instruction(instr, a, b);
        }
        Answer::Num(grid.count_on_lights() as isize)
    }
}

