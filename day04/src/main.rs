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

fn part1(lines: &Vec<String>) {
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
                0
            } else {
                2_u32.pow(n_matching_numbers as u32 - 1)
            }
        })
        .sum::<u32>();

    println!("Part1: {}", pile_worth);
}

// Isn't this similar to fibonacci numbers?
fn part2(lines: &Vec<String>) {
    let number_of_scratchcards = lines
        .iter()
        .filter_map(|line| line.split_once("|"))
        .map(|(a, b)| {
            let winning_numbers = &numbers(a)[1..];
            let my_numbers = &numbers(b)[0..];

            let n_matching_numbers = my_numbers
                .iter()
                .filter(|n| winning_numbers.contains(n))
                .count();

            return n_matching_numbers;
        })
        .rev()
        .fold(vec![], |mut ns, n| {
            let cards = 1 + ns.iter().skip(ns.len().saturating_sub(n)).sum::<usize>();
            ns.push(cards);
            ns
        })
        .iter()
        .sum::<usize>();

    println!("Part1: {}", number_of_scratchcards);
}

fn main() {
    let lines = io::stdin().lines().map(|l| l.unwrap()).collect::<Vec<_>>();
    part1(&lines);
    part2(&lines)
}
