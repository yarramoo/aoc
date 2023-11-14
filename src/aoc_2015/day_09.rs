use std::{collections::HashMap, str::FromStr};

use crate::{IChallenge, Answer};

pub const DATA_PATH: &str = "src/aoc_2015/input/day_09";

struct Route(String, String, usize);

impl FromStr for Route {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let words = s.split_ascii_whitespace().collect::<Vec<_>>();
        let a = words[0].to_string();
        let b = words[2].to_string();
        let distance = words[4].parse::<usize>().unwrap();
        Ok(Route(a, b, distance))
    }
}

pub struct Challenge {
    distance_matrix: HashMap<(usize, usize), usize>,
    all_routes: Vec<Vec<usize>>,
}

macro_rules! vec_get_item_index {
    ($vec:ident, $item:expr) => {
        $vec.iter().position(|e| e == &$item).unwrap() 
    };
}

impl Challenge {
    fn get_route_length(&self, route: &[usize]) -> usize {
        let mut total_length = 0;
        for window in route.windows(2) {
            let pair = window.iter().collect::<Vec<_>>();
            let distance = self.distance_matrix.get(&(*pair[0], *pair[1])).unwrap();
            total_length += distance;
        }
        total_length
    }

    fn all_routes(cities: &Vec<String>) -> Vec<Vec<usize>> 
    {
        fn search_recursive(cities: &Vec<usize>, used: &mut Vec<usize>, found_routes: &mut Vec<Vec<usize>>) {
            if used.len() == cities.len() {
                found_routes.push(used.clone());
                return;
            }
    
            for city in cities {
                if !used.contains(city) {
                    used.push(*city);
                    search_recursive(cities, used, found_routes);
                    let _ = used.pop();
                }
            }
        }
        let cities = Vec::from_iter(0..cities.len());
        let mut used: Vec<usize> = Vec::with_capacity(cities.len());
        let mut found_routes = Vec::new();
        search_recursive(&cities, &mut used, &mut found_routes);
        found_routes
    }

    fn find_min_route(&self) -> (Vec<usize>, usize) {
        let best_route = self.all_routes
            .iter()
            .min_by(|&x, &y| self.get_route_length(x).cmp(&self.get_route_length(y)))
            .unwrap()
            .to_owned();
        let cost = self.get_route_length(&best_route);
        (best_route, cost)
    }

    fn find_max_route(&self) -> (Vec<usize>, usize) {
        let worst_route = self.all_routes
            .iter()
            .max_by(|&x, &y| self.get_route_length(x).cmp(&self.get_route_length(y)))
            .unwrap()
            .to_owned();
        let cost = self.get_route_length(&worst_route);
        (worst_route, cost)
    }
}

impl IChallenge for Challenge {
    fn parse(input: &str) -> Self {
        let mut distance_matrix = HashMap::new();
        let mut cities = Vec::new();
        for l in input.lines() {
            let route = Route::from_str(l).unwrap();
            if !cities.contains(&route.0) {
                cities.push(route.0.clone());
            }
            if !cities.contains(&route.1) {
                cities.push(route.1.clone());
            }

            let city_1_index = vec_get_item_index!(cities, route.0);
            let city_2_index = vec_get_item_index!(cities, route.1);

            distance_matrix.insert(
                (city_1_index, city_2_index),
                route.2
            );
            distance_matrix.insert(
                (city_2_index, city_1_index),
                route.2
            );
        }
        let all_routes = Challenge::all_routes(&cities);

        Challenge { distance_matrix, all_routes }
    }

    fn solve_p1(&self) -> Answer {
        let (best_route, cost) = self.find_min_route();
        println!("{:?}", best_route);
        Answer::Num(cost as isize)
    }

    fn solve_p2(&self) -> Answer {
        let (best_route, cost) = self.find_max_route();
        println!("{:?}", best_route);
        Answer::Num(cost as isize)
    }
}