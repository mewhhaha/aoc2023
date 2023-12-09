use std::{collections::HashMap, io};

use num::Integer;

fn count_moves<'a>(
    nodes: &'a HashMap<&str, (&str, &str)>,
    start_node: &'a str,
    end_node: &'a str,
    instructions: std::str::Chars<'_>,
) -> usize {
    let mut moves = 0;
    let mut node = start_node;
    for instruction in instructions.cycle() {
        let (left, right) = nodes.get(node).expect("There to be a node");
        node = match instruction {
            'L' => left,
            'R' => right,
            _ => panic!("Unknown instruction"),
        };

        moves += 1;
        if node.ends_with(end_node) {
            break;
        }
    }

    moves
}

fn make_graph(lines: &Vec<String>) -> HashMap<&str, (&str, &str)> {
    lines
        .iter()
        .skip(2)
        .map(|l| {
            let id = l.get(0..=2).expect("There to be a node name");

            let left = l.get(7..=9).expect("There to be a left node");
            let right = l.get(12..=14).expect("There to be right node");

            return (id, (left, right));
        })
        .collect::<HashMap<_, _>>()
}

fn part1(lines: &Vec<String>) {
    let instructions = &lines[0];

    let nodes = make_graph(lines);

    let moves = count_moves(&nodes, "AAA", "ZZZ", instructions.chars());

    println!("Part1: {}", moves);
}

fn part2(lines: &Vec<String>) {
    let instructions = &lines[0];

    let nodes = make_graph(lines);

    let state = nodes
        .keys()
        .filter(|k| k.ends_with("A"))
        .collect::<Vec<_>>();

    let mut q: Option<usize> = None;

    for start in state {
        let m = count_moves(&nodes, start, "Z", instructions.chars());

        q = Some(q.map_or(m, |w| w.lcm(&m)));
    }

    println!("Part2: {}", q.unwrap());
}

fn main() {
    let lines = io::stdin().lines().map(|l| l.unwrap()).collect::<Vec<_>>();
    part1(&lines);
    part2(&lines);
}
