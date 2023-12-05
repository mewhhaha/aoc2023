#![feature(slice_group_by)]
use std::io;

fn numbers(line: &str) -> Vec<i32> {
    let chars = line.chars().collect::<Vec<_>>();
    chars
        .group_by(|a, b| a.is_ascii_digit() && b.is_ascii_digit())
        .filter_map(|group| {
            if group[0].is_ascii_digit() {
                group.into_iter().collect::<String>().parse::<i32>().ok()
            } else {
                None
            }
        })
        .collect()
}

fn part1(lines: Vec<String>) {
    let pile_worth = lines
        .iter()
        .filter_map(|line| line.split_once("|"))
        .map(|(a, b)| {
            let winning_numbers = &numbers(a)[1..];
            let my_numbers = &numbers(b)[0..];

            let n_matching_numbers = my_numbers
                .iter()
                .filter(|n| winning_numbers.contains(n))
                .count();

            if n_matching_numbers == 0 {
                return 0;
            }

            (2 as u32).pow(n_matching_numbers as u32 - 1)
        })
        .sum::<u32>();

    println!("Part1: {}", pile_worth);
}

fn part2() {}

fn main() {
    let lines = io::stdin().lines().map(|l| l.unwrap()).collect::<Vec<_>>();
    part1(lines);
}
