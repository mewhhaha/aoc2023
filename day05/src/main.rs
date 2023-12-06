#![feature(slice_group_by)]
use std::{io, process::id};

fn starts_with_digit(s: &String) -> bool {
    s.chars().next().map_or(false, |c| c.is_ascii_digit())
}

fn part1(lines: &Vec<String>) {
    let initial_seeds = lines[0]
        .split_ascii_whitespace()
        .filter_map(|s| s.parse::<i64>().ok())
        .collect::<Vec<_>>();

    let locations = lines
        .group_by(|a, b| {
            return starts_with_digit(a) && starts_with_digit(b);
        })
        .filter(|group| starts_with_digit(&group[0]))
        .fold(initial_seeds, |seeds, group| {
            let numbers = group
                .iter()
                .map(|l| {
                    match l
                        .split_whitespace()
                        .filter_map(|s| s.parse::<i64>().ok())
                        .collect::<Vec<_>>()
                        .as_slice()
                    {
                        [destination, start, start_offset] => {
                            (*start, start + start_offset, start - destination)
                        }
                        _ => panic!("Expected 3 numbers"),
                    }
                })
                .collect::<Vec<_>>();

            seeds
                .into_iter()
                .map(|s| {
                    numbers
                        .iter()
                        .find_map(|(start, end, offset)| {
                            if s >= *start && s < *end {
                                Some(s - offset)
                            } else {
                                None
                            }
                        })
                        .unwrap_or(s)
                })
                .collect()
        });

    let smallest_location = locations
        .iter()
        .min()
        .expect("There to be a smallest location");

    println!("Part1: {:?}", smallest_location);
}

fn part2(lines: &Vec<String>) {}

fn main() {
    let lines = io::stdin().lines().map(|l| l.unwrap()).collect::<Vec<_>>();
    part1(&lines);
    part2(&lines)
}
