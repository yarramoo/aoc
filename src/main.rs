use std::time::Instant;
use std::fs::read_to_string;

mod aoc_2015;

macro_rules! run_and_time {
    ($f:expr, $s:expr) => {{
        let start = Instant::now();
        let result = $f;
        let time = start.elapsed();
        (result, time)
    }};
}


fn main() {
    use aoc_2015::day_08 as day;
    let builder = ChallengeBuilder::new(day::DATA_PATH, day::Challenge::parse);
    
    let (challenge, parse_time) = run_and_time!(builder.parse(), "Parsing");
    println!("Parsing: {:?}", parse_time);
    

    let (answer_1, answer_1_time) = run_and_time!(challenge.solve_p1(), "Part 1");
    println!("Part 1: {:?}; {:?}", answer_1, answer_1_time);

    let (answer_2, answer_2_time) = run_and_time!(challenge.solve_p2(), "Part 2");
    println!("Part 2: {:?}; {:?}", answer_2, answer_2_time);

    for _challenge in aoc_2015::days() {
        // println!("{:?}", challenge.solve_p1());
    }
}

fn load_data(input_path: &str) -> String {
    read_to_string(input_path).expect("Could not read data")
}


pub trait IChallenge {
    fn parse(data: &str) -> Self where Self: Sized;
    fn solve_p1(&self) -> Answer;
    fn solve_p2(&self) -> Answer;
}

pub trait ChallengeInstance: IChallenge + Sized {}
impl<T: IChallenge + Sized> ChallengeInstance for T {}


pub struct ChallengeBuilder<T> 
where
    T: ChallengeInstance
{
    data: String,
    parsing_function: fn(&str) -> T,
}

impl<T> ChallengeBuilder<T>
where
    T: ChallengeInstance
{
    fn new(
        input_path: &str,
        parsing_function: fn(&str) -> T,
    ) -> Self 
    {
        ChallengeBuilder { data: load_data(input_path), parsing_function }
    }

    // fn with_data(data: String, parsing_function: fn(&str) -> T) -> Self {
    //     ChallengeBuilder { data, parsing_function }
    // }

    fn parse(self) -> T {
        (self.parsing_function)(&self.data)
    }
}


#[derive(Debug)]
pub enum Answer {
    Num(isize),
    // Float(f64),
    // String(String),
}


