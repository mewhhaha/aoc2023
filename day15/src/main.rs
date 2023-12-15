use std::io;

fn hash_lens(input: &str) -> usize {
    let mut value = 0;
    for c in input.chars() {
        let code = c as usize;
        value += code;
        value *= 17;
        value %= 256;
    }

    value
}

fn part1(lines: &Vec<String>) {
    let steps = lines[0].split(',');

    let sum = steps.into_iter().map(hash_lens).sum::<usize>();

    println!("Part1: {}", sum);
}

enum Operation {
    Add(String, u32),
    Sub(String),
}

fn parse_lens(step: &str) -> Operation {
    let mut key = String::new();
    let mut sign = '0';
    let mut value = String::new();

    for c in step.chars() {
        match c {
            '0'..='9' => {
                value.push(c);
            }
            '-' | '=' => {
                sign = c;
            }
            _ => {
                key.push(c);
            }
        }
    }

    if sign == '=' {
        Operation::Add(key, value.parse::<u32>().unwrap())
    } else {
        Operation::Sub(key)
    }
}

fn part2(lines: &Vec<String>) {
    let steps = lines[0].split(',');

    let mut hashmap: Vec<Vec<(String, u32)>> = vec![vec![]; 256];

    for step in steps {
        let operation = parse_lens(&step);

        match operation {
            Operation::Add(key, value) => {
                let index = hash_lens(&key) as usize;
                let position = hashmap[index].iter().position(|(x, _)| *x == key);
                if let Some(position) = position {
                    hashmap[index][position].1 = value;
                } else {
                    hashmap[index].push((key, value));
                }
            }
            Operation::Sub(key) => {
                let index = hash_lens(&key) as usize;
                let position = hashmap[index].iter().position(|(x, _)| *x == key);

                if let Some(position) = position {
                    hashmap[index].remove(position);
                }
            }
        }
    }

    let mut sum = 0;
    for (i, boxes) in hashmap.iter().enumerate() {
        for (j, (_, value)) in boxes.iter().enumerate() {
            sum += (i as u32 + 1) * (j as u32 + 1) * value;
        }
    }

    println!("Part2: {}", sum);
}

fn main() {
    let lines = io::stdin().lines().map(|l| l.unwrap()).collect::<Vec<_>>();
    part1(&lines);
    part2(&lines);
}
