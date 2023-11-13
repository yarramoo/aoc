use crate::{IChallenge, load_data};

pub mod day_01;
pub mod day_02;
pub mod day_03;
pub mod day_04;
pub mod day_05;
pub mod day_06;
pub mod day_07;
pub mod day_08;

pub fn days() -> Vec<Box<dyn IChallenge>> {
    vec![
        Box::new(day_01::Challenge::parse(&load_data(day_01::DATA_PATH))),
        Box::new(day_02::Challenge::parse(&load_data(day_02::DATA_PATH))),
        Box::new(day_03::Challenge::parse(&load_data(day_03::DATA_PATH))),
        Box::new(day_04::Challenge::parse(&load_data(day_04::DATA_PATH))),
        Box::new(day_05::Challenge::parse(&load_data(day_05::DATA_PATH))),
        Box::new(day_06::Challenge::parse(&load_data(day_06::DATA_PATH))),
        Box::new(day_07::Challenge::parse(&load_data(day_07::DATA_PATH))),
    ]
}