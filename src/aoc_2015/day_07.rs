use std::{str::FromStr, collections::HashMap, hash::Hash};
use topological_sort::TopologicalSort;

use crate::{IChallenge, Answer};

pub const DATA_PATH: &str = "src/aoc_2015/input/day_07";


pub struct Challenge {
    data: Vec<Connection>,
}

impl IChallenge for Challenge {
    fn parse(input: &str) -> Self {
        let connections = input
            .lines()
            .map(|l| Connection::from_str(l).unwrap())
            .collect::<Vec<_>>();

        let mut ts = TopologicalSort::<Connection>::new();
        for i in 0..connections.len() {
            for j in i+1..connections.len() {
                if let Some(partial_ord) = connections[i].partial_cmp(&connections[j]) {
                    match partial_ord {
                        std::cmp::Ordering::Less => ts.add_dependency(connections[i].clone(), connections[j].clone()),
                        std::cmp::Ordering::Equal => {},
                        std::cmp::Ordering::Greater => ts.add_dependency(connections[j].clone(), connections[i].clone()),
                    }
                }
            }
        }

        let mut data = Vec::new();
        while !ts.is_empty() {
            let batch = ts.pop_all();
            data.extend(batch);
        }
        
        Self { data }
    }

    fn solve_p1(&self) -> Answer {
        let mut mapping = HashMap::new();
        for conn in &self.data {
            let result_value = conn.eval(&mapping);
            mapping.insert(conn.wire.clone(), result_value);
        }
        Answer::Num(*mapping.get(&"a".to_string()).unwrap() as isize)
    }

    fn solve_p2(&self) -> Answer {
        let b_init: WireValue = 46065;
        let mut mapping = HashMap::new();
        mapping.insert("b".to_string(), b_init);
        for conn in &self.data {
            if conn.wire == "b" { continue; }
            let result_value = conn.eval(&mapping);
            mapping.insert(conn.wire.clone(), result_value);
        }
        Answer::Num(*mapping.get(&"a".to_string()).unwrap() as isize)
    }
}

type WireValue = u16;
type WireId = String;

#[derive(Debug, PartialEq, Eq, Clone, Hash)]
enum Literal {
    WireId(WireId),
    WireValue(WireValue),
}

impl Literal {
    fn get_value(&self, mapping: &HashMap<WireId, WireValue>) -> WireValue {
        match self {
            Literal::WireId(wid) => *mapping.get(wid).unwrap(),
            Literal::WireValue(val) => *val,
        }
    }
}

impl FromStr for Literal {
    type Err = String;
    fn from_str(value: &str) -> Result<Self, Self::Err> {
        if let Ok(n) = value.parse::<WireValue>() {
            return Ok(Self::WireValue(n));
        }
        else if let Ok(wid) = value.parse::<WireId>() {
            return Ok(Self::WireId(wid));
        }
        Err(format!("Could not parse Literal: {}", value))
    }
}

impl From<&str> for Literal {
    fn from(value: &str) -> Self {
        if let Ok(literal) = value.parse() {
            return literal;
        } 
        panic!("{}", format!("Could not parse Literal: {}", value));
    }
}

impl From<usize> for Literal {
    fn from(value: usize) -> Self {
        Literal::WireValue(value as WireValue)
    }
}

#[allow(clippy::upper_case_acronyms)]
#[derive(Debug, PartialEq, Eq, Clone, Hash)]
enum Gate {
    AND(Literal, Literal),
    OR(Literal, Literal),
    LSHIFT(Literal, Literal),
    RSHIFT(Literal, Literal),
    NOT(Literal),
}

impl Gate {
    fn new_binary(gate: &str, a: Literal, b: Literal) -> Self {
        match gate {
            "AND" => Self::AND(a, b),
            "OR"  => Self::OR(a, b),
            "LSHIFT" => Self::LSHIFT(a, b),
            "RSHIFT" => Self::RSHIFT(a, b),
            _ => panic!("Cannot make a gate out of: {}", gate)
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Hash)]
enum Input {
    Gate(Gate),
    Literal(Literal),
}

#[derive(Debug, PartialEq, Eq, Clone,Hash)]
struct Connection {
    input: Input, 
    wire: WireId,
}

impl Connection {
    fn eval(&self, mapping: &HashMap<WireId, WireValue>) -> WireValue {
        match &self.input {
            Input::Gate(gate) => match &gate {
                Gate::AND(a, b)    => a.get_value(mapping) & b.get_value(mapping),
                Gate::OR(a, b)     => a.get_value(mapping) | b.get_value(mapping),
                Gate::LSHIFT(a, b) => a.get_value(mapping) << b.get_value(mapping),
                Gate::RSHIFT(a, b) => a.get_value(mapping) >> b.get_value(mapping),
                Gate::NOT(a)       => !a.get_value(mapping),
            },
            Input::Literal(literal) => match &literal {
                Literal::WireId(wid)  => *mapping.get(wid).unwrap(),
                Literal::WireValue(v) => *v,
            },
        }
    }

    fn new(input: Input, wire: WireId) -> Self {
        Connection { input, wire }
    }

    fn uses(&self) -> Vec<&WireId> {

        macro_rules! push_if_WireId {
            ($vec:ident, $literal:ident) => {
                if let Literal::WireId(wid) = $literal {
                    $vec.push(wid);
                }
            };
        }

        let mut result = Vec::new();
        match &self.input {
            Input::Gate(gate) => {
                match gate {
                    Gate::AND(a, b) => {
                        push_if_WireId!(result, a);
                        push_if_WireId!(result, b);
                    },
                    Gate::OR(a, b) => {
                        push_if_WireId!(result, a);
                        push_if_WireId!(result, b);
                    },
                    Gate::LSHIFT(a, b) => {
                        push_if_WireId!(result, a);
                        push_if_WireId!(result, b);
                    },
                    Gate::RSHIFT(a, b) => {
                        push_if_WireId!(result, a);
                        push_if_WireId!(result, b);
                    },
                    Gate::NOT(a) => push_if_WireId!(result, a),
                };
            },
            Input::Literal(literal) => {
                push_if_WireId!(result, literal);
            },
        };
        result
    }

    fn defines(&self) -> &WireId {
        &self.wire
    }
}

impl PartialOrd for Connection {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        let self_before: bool = other.uses().contains(&self.defines());
        let other_before: bool = self.uses().contains(&other.defines());

        if self_before && other_before {
            panic!("Circular dependancy detected!!");
        }

        if self_before {
            return Some(std::cmp::Ordering::Less);
        }
        if other_before {
            return Some(std::cmp::Ordering::Greater);
        }
        None
    }
}

impl FromStr for Connection {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let comps: Vec<&str> = s.split("->").collect();
        let target: WireId = comps[1].trim().to_string();
        let source: Vec<&str> = comps[0].trim().split_ascii_whitespace().collect();
        // Have to parse the left hand side now
        // Cases
        //  Direct WireId assign (source will be len() == 1 and lhs will be WireId)
        //  Direct WireValue assign (source will be len() == 1 and lhs will be WireValue)
        //  Gate (source len() > 1)
        //      NOT (source will be len() == 2)
        //      AND / OR / LSHIFT / RSHIFT (will have to discriminate by source[1])
        if source.len() == 1 {
            let literal = source[0].parse::<Literal>();
            if let Ok(literal) = literal {
                return Ok(
                    Connection::new(
                        Input::Literal(literal), 
                        target
                    )
                );
            } else {
                return Err(literal.err().unwrap());
            }
        }

        if source.len() == 2 {
            if let Ok(literal) = source[1].parse::<Literal>() {
                return Ok(
                    Connection::new(
                        Input::Gate(Gate::NOT(literal)),
                        target
                    )
                );
            }
        }

        if let (Ok(a), Ok(b)) = (source[0].parse::<Literal>(), source[2].parse::<Literal>()) {
            return Ok(
                Connection::new(
                    Input::Gate(
                        Gate::new_binary(source[1], a, b)
                    ),
                    target
                )
            );
        }
        
        Err(format!("Could not parse Connection: {}", s))
    }
}

#[test]
fn test_conn_ord() {
    let a = "lx -> a";
    let b = "b -> c";
    let c_a = Connection::from_str(a).unwrap();
    let c_b = Connection::from_str(b).unwrap();
    assert_eq!(
        None,
        c_a.partial_cmp(&c_b)
    );

    let a = "lx -> a";
    let b = "b -> lx";
    let c_a = Connection::from_str(a).unwrap();
    let c_b = Connection::from_str(b).unwrap();
    assert_eq!(
        Some(std::cmp::Ordering::Greater),
        c_a.partial_cmp(&c_b)
    );


    let a = "lx -> a";
    let b = "b -> lx";
    let c_a = Connection::from_str(a).unwrap();
    let c_b = Connection::from_str(b).unwrap();
    assert_eq!(
        Some(std::cmp::Ordering::Less),
        c_b.partial_cmp(&c_a)
    );
}

#[test]
fn test_conn_uses() {
    let s = "lx -> a";
    let c = Connection::from_str(s).unwrap();
    assert_eq!(
        vec![&"lx".to_string()],
        c.uses()
    );

    let s = "lr AND lt -> lu";
    let c = Connection::from_str(s).unwrap();
    assert_eq!(
        vec![&"lr".to_string(), &"lt".to_string()],
        c.uses()
    );
}

#[test]
fn test_conn_from_str() {
    let s = "0 -> bo";
    let c = Connection::from_str(s).unwrap();
    assert_eq!(
        c, 
        Connection::new(
            Input::Literal(0.into()), 
            "bo".to_string()
        )
    );

    let s = "lx -> a";
    let c = Connection::from_str(s).unwrap();
    assert_eq!(
        c, 
        Connection::new(
            Input::Literal("lx".into()),
            "a".to_string()
        )
    );

    let s = "NOT ax -> ay";
    let c = Connection::from_str(s).unwrap();
    assert_eq!(
        c, 
        Connection::new(
            Input::Gate(Gate::NOT("ax".into())),
            "ay".to_string() 
        )
    );

    let s = "bn RSHIFT 2 -> bo";
    let c = Connection::from_str(s).unwrap();
    assert_eq!(
        c, 
        Connection::new( 
            Input::Gate(Gate::RSHIFT("bn".into(), 2.into())), 
            "bo".to_string()
        )
    );

    let s = "hb LSHIFT 1 -> hv";
    let c = Connection::from_str(s).unwrap();
    assert_eq!(
        c, 
        Connection::new( 
            Input::Gate(Gate::LSHIFT("hb".into(), 1.into())), 
            "hv".to_string()
        )
    );

    let s = "lr AND lt -> lu";
    let c = Connection::from_str(s).unwrap();
    assert_eq!(
        c, 
        Connection::new(
            Input::Gate(Gate::AND("lr".into(), "lt".into())), 
            "lu".to_string()
        )
    );

    let s = "dy OR ej -> ek";
    let c = Connection::from_str(s).unwrap();
    assert_eq!(
        c, 
        Connection::new(
            Input::Gate(Gate::OR("dy".into(), "ej".into())), 
            "ek".to_string()
        )
    );
}


