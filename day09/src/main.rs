use std::io;

fn predict_next_number(sequence: Vec<i64>) -> i64 {
    if sequence.iter().all(|n| Some(n) == sequence.first()) {
        return *sequence.last().unwrap();
    }

    let mut next_sequence = vec![];

    for i in 0..(sequence.len() - 1) {
        let current = sequence[i];
        let next = sequence[i + 1];

        next_sequence.push(next - current);
    }

    let next_number = predict_next_number(next_sequence);

    return sequence.last().unwrap() + next_number;
}

fn part1(lines: &Vec<String>) {
    let sequences = lines.iter().map(|l| {
        l.split_ascii_whitespace()
            .map(|n| n.parse::<i64>().expect("It to be a number"))
            .collect::<Vec<_>>()
    });

    let value = sequences.map(predict_next_number).sum::<i64>();

    println!("Part1: {}", value);
}

fn part2(lines: &Vec<String>) {
    let sequences = lines.iter().map(|l| {
        l.split_ascii_whitespace()
            .map(|n| n.parse::<i64>().expect("It to be a number"))
            .rev()
            .collect::<Vec<_>>()
    });

    let value = sequences.map(predict_next_number).sum::<i64>();

    println!("Part2: {}", value);
}

fn main() {
    let lines = io::stdin().lines().map(|l| l.unwrap()).collect::<Vec<_>>();
    part1(&lines);
    part2(&lines);
}
