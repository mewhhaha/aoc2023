#![feature(slice_group_by)]
use std::{
    collections::{HashMap, HashSet},
    io,
    ops::Add,
    str::Chars,
};

enum Category {
    X,
    M,
    A,
    S,
}
enum Comparison {
    LT,
    GT,
}

enum Result {
    Forwarded(String),
    Accepted,
    Rejected,
}

enum Condition {
    If(Category, Comparison, i64, Result),
    Else(Result),
}

struct Sorter {
    key: String,
    conditions: Vec<Condition>,
}

struct Part {
    x: i64,
    m: i64,
    a: i64,
    s: i64,
}

fn parse_sorter(line: &String) -> Sorter {
    // Example of input mjv{x<2089:R,x>2415:R,x<2280:A,R}
    let (name, rest) = line.split_once('{').unwrap();
    let (raw_conditions, _) = rest.split_once('}').unwrap();
    let split_raw_conditions = raw_conditions.split(',').collect::<Vec<_>>();

    fn parse_if_condition(line: &str) -> Condition {
        let mut rest = line.chars().skip(1);
        let category = match line.chars().next().unwrap() {
            'x' => Category::X,
            'm' => Category::M,
            'a' => Category::A,
            's' => Category::S,
            _ => panic!("Invalid category"),
        };

        let comparison = match rest.next().unwrap() {
            '<' => Comparison::LT,
            '>' => Comparison::GT,
            _ => panic!("Invalid comparison"),
        };

        let mut raw_value = String::new();
        while let Some(c) = rest.next() {
            if c == ':' {
                break;
            }

            raw_value.push(c)
        }

        let value = raw_value.parse::<i64>().unwrap();

        let raw_result = rest.collect::<String>();
        let result = match raw_result.as_str() {
            "R" => Result::Rejected,
            "A" => Result::Accepted,
            _ => Result::Forwarded(raw_result),
        };

        Condition::If(category, comparison, value, result)
    }

    fn parse_condition(line: &str) -> Condition {
        if line.contains(":") {
            return parse_if_condition(line);
        }

        match line {
            "A" => return Condition::Else(Result::Accepted),
            "R" => return Condition::Else(Result::Rejected),
            _ => Condition::Else(Result::Forwarded(line.to_string())),
        }
    }

    let conditions = split_raw_conditions
        .into_iter()
        .map(parse_condition)
        .collect::<Vec<_>>();

    Sorter {
        key: name.to_string(),
        conditions,
    }
}

fn parse_part(line: &String) -> Part {
    let [x, m, a, s]: [i64; 4] = line
        .split(',')
        .map(|s| {
            s.chars()
                .filter(|c| c.is_digit(10))
                .collect::<String>()
                .parse::<i64>()
                .unwrap()
        })
        .collect::<Vec<_>>()
        .try_into()
        .expect("There to be four columns");

    Part { x, m, a, s }
}

fn sum_of_xmas(p: Part) -> i64 {
    p.x + p.m + p.a + p.s
}

fn part1(lines: &Vec<String>) {
    let (sorter_lines, part_lines) = lines.split_at(lines.iter().position(|l| l == "").unwrap());

    let sorters = sorter_lines
        .iter()
        .map(parse_sorter)
        .map(|s| (s.key.clone(), s))
        .collect::<HashMap<_, _>>();
    let parts = part_lines[1..].iter().map(parse_part).collect::<Vec<_>>();

    let mut sum = 0;

    for part in parts {
        let mut result = &Result::Forwarded("in".to_string());

        while let Result::Forwarded(sorter_key) = result {
            let sorter = sorters.get(sorter_key).unwrap();

            for condition in &sorter.conditions {
                let maybe_result = match condition {
                    Condition::If(c, comparison, value, result) => {
                        let part_value = match c {
                            Category::X => part.x,
                            Category::M => part.m,
                            Category::A => part.a,
                            Category::S => part.s,
                        };

                        let passed = match comparison {
                            Comparison::LT => part_value < *value,
                            Comparison::GT => part_value > *value,
                        };

                        if passed {
                            Some(result)
                        } else {
                            None
                        }
                    }
                    Condition::Else(result) => Some(result),
                };
                if let Some(r) = maybe_result {
                    result = r;
                    break;
                }
            }
        }

        if let Result::Accepted = result {
            sum += sum_of_xmas(part);
        }
    }

    println!("Part1: {}", sum);
}

fn part2(lines: &Vec<String>) {
    println!("Part2: {}", "");
}

fn main() {
    let lines = io::stdin().lines().map(|l| l.unwrap()).collect::<Vec<_>>();
    part1(&lines);
    part2(&lines);
}
