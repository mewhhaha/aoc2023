#![feature(slice_group_by)]
use std::{collections::HashMap, io, str::FromStr};

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
enum Category {
    X = 0,
    M = 1,
    A = 2,
    S = 3,
}

impl FromStr for Category {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "x" => Ok(Category::X),
            "m" => Ok(Category::M),
            "a" => Ok(Category::A),
            "s" => Ok(Category::S),
            _ => Err("Invalid category"),
        }
    }
}
enum Comparison {
    LT,
    GT,
}

enum To {
    Forwarded(String),
    Accepted,
    Rejected,
}

enum Condition {
    If(Category, Comparison, i64, To),
    Else(To),
}

struct Sorter {
    key: String,
    conditions: Vec<Condition>,
}

type Part = [i64; 4];

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
            "R" => To::Rejected,
            "A" => To::Accepted,
            _ => To::Forwarded(raw_result),
        };

        Condition::If(category, comparison, value, result)
    }

    fn parse_condition(line: &str) -> Condition {
        if line.contains(":") {
            return parse_if_condition(line);
        }

        match line {
            "A" => return Condition::Else(To::Accepted),
            "R" => return Condition::Else(To::Rejected),
            _ => Condition::Else(To::Forwarded(line.to_string())),
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

    [x, m, a, s]
}

fn sum_of_xmas(p: Part) -> i64 {
    p[Category::X as usize]
        + p[Category::M as usize]
        + p[Category::A as usize]
        + p[Category::S as usize]
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
        let mut result = &To::Forwarded("in".to_string());

        while let To::Forwarded(sorter_key) = result {
            let sorter = sorters.get(sorter_key).unwrap();

            for condition in &sorter.conditions {
                let maybe_result = match condition {
                    Condition::If(c, comparison, value, result) => {
                        let part_value = part[*c as usize];

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

        if let To::Accepted = result {
            sum += sum_of_xmas(part);
        }
    }

    println!("Part1: {}", sum);
}

// Just using vec for simplicity
// Could be a range or an (i64, i64) but because the size is just 1-4000
// then might as well use all the associated functions with vec
type RangedPart = [Vec<i64>; 4];

fn sum_of_xmas_ranged(part: &RangedPart) -> i64 {
    part[Category::X as usize].len() as i64
        * part[Category::M as usize].len() as i64
        * part[Category::A as usize].len() as i64
        * part[Category::S as usize].len() as i64
}

fn part2(lines: &Vec<String>) {
    let (sorter_lines, _) = lines.split_at(lines.iter().position(|l| l == "").unwrap());

    fn count_combinations(sorters: &HashMap<String, Sorter>, part: RangedPart, result: &To) -> i64 {
        if sum_of_xmas_ranged(&part) == 0 {
            return 0;
        }

        let sorter: &Sorter;
        match result {
            To::Accepted => return sum_of_xmas_ranged(&part),
            To::Rejected => return 0,
            To::Forwarded(s) => sorter = sorters.get(s).unwrap(),
        }

        let mut sum = 0;

        let mut failed_part = part.clone();

        for condition in &sorter.conditions {
            match condition {
                Condition::If(c, comparison, value, result) => {
                    let part_filter = |v: &&i64| match comparison {
                        Comparison::LT => *v < value,
                        Comparison::GT => *v > value,
                    };

                    let mut passed_part = failed_part.clone();

                    let key = *c as usize;
                    let (passed_value, failed_value) =
                        failed_part[key].iter().partition(part_filter);

                    failed_part[key] = failed_value;
                    passed_part[key] = passed_value;

                    sum += count_combinations(sorters, passed_part, result);
                }
                Condition::Else(result) => {
                    sum += count_combinations(sorters, failed_part, result);
                    break;
                }
            };
        }

        sum
    }

    let sorters = sorter_lines
        .iter()
        .map(parse_sorter)
        .map(|s| (s.key.clone(), s))
        .collect::<HashMap<_, _>>();

    let one_to_4000 = (1..=4000).collect::<Vec<_>>();

    let part: RangedPart = [
        one_to_4000.clone(),
        one_to_4000.clone(),
        one_to_4000.clone(),
        one_to_4000.clone(),
    ];

    let result = &To::Forwarded("in".to_string());
    let sum = count_combinations(&sorters, part, result);

    println!("Part2: {}", sum);
}

fn main() {
    let lines = io::stdin().lines().map(|l| l.unwrap()).collect::<Vec<_>>();
    part1(&lines);
    part2(&lines);
}
