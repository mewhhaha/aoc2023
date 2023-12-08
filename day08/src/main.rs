use std::{cmp::Ordering, collections::HashMap, io};

fn part1(lines: &Vec<String>) {
    let instructions = lines[0].chars().cycle();

    let nodes = lines
        .iter()
        .skip(2)
        .map(|l| {
            let id = l.get(0..=2).expect("There to be a node name");

            let left = l.get(7..=9).expect("There to be a left node");
            let right = l.get(12..=14).expect("There to be right node");

            return (id, (left, right));
        })
        .collect::<HashMap<_, _>>();

    let mut node = "AAA";
    let mut moves = 0;

    for instruction in instructions {
        if node == "ZZZ" {
            break;
        }

        let (left, right) = nodes.get(node).expect("There to be a node");
        node = match instruction {
            'L' => left,
            'R' => right,
            _ => panic!("Unknown instruction"),
        };
        moves += 1;
    }

    println!("Part1: {}", moves);
}

fn part2(lines: &Vec<String>) {
    println!("Part2: {}", "");
}

fn main() {
    let lines = io::stdin().lines().map(|l| l.unwrap()).collect::<Vec<_>>();
    part1(&lines);
    part2(&lines)
}
