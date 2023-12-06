#![feature(slice_group_by)]
use std::io;

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
            let mapper_ranges = group
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
                    mapper_ranges
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

fn part2(lines: &Vec<String>) {
    let initial_ranges = lines[0]
        .split_ascii_whitespace()
        .filter_map(|s| s.parse::<i64>().ok())
        .collect::<Vec<_>>()
        .chunks(2)
        .map(|c| (c[0], c[0] + c[1]))
        .collect::<Vec<_>>();

    let locations = lines
        .group_by(|a, b| {
            return starts_with_digit(a) && starts_with_digit(b);
        })
        .filter(|group| starts_with_digit(&group[0]))
        .fold(initial_ranges, |ranges, group| {
            let mapper_ranges = group
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

            let mut result = vec![];
            let mut buffer = ranges.clone();

            for (mapper_start, mapper_end, offset) in mapper_ranges.into_iter() {
                let mut tmp = vec![];
                for (range_start, range_end) in buffer.into_iter() {
                    let is_complete_overlap =
                        mapper_start <= range_start && mapper_end >= range_end;
                    let is_left_hand_overlap = mapper_start <= range_start
                        && mapper_end >= range_start
                        && mapper_end < range_end;
                    let is_right_hand_overlap = mapper_start >= range_start
                        && mapper_start < range_end
                        && mapper_end >= range_end;
                    let is_within = mapper_start > range_start && mapper_end < range_end;
                    if is_complete_overlap {
                        result.push((range_start - offset, range_end - offset));
                    } else if is_within {
                        let left = (range_start, mapper_start);
                        let right = (mapper_end, range_end);

                        tmp.push(left);
                        tmp.push(right);

                        result.push((mapper_start - offset, mapper_end - offset))
                    } else if is_left_hand_overlap {
                        let right = (mapper_end, range_end);
                        tmp.push(right);

                        result.push((range_start - offset, mapper_end - offset))
                    } else if is_right_hand_overlap {
                        let left = (range_start, mapper_start);
                        tmp.push(left);

                        result.push((mapper_start - offset, range_end - offset))
                    } else {
                        tmp.push((range_start, range_end));
                    }
                }

                buffer = tmp;
            }

            result.extend(buffer);

            result
        });

    let smallest_location = locations
        .iter()
        .map(|(start, _)| start)
        .min()
        .expect("There to be a smallest location");

    println!("Part2: {:?}", smallest_location);
}

fn main() {
    let lines = io::stdin().lines().map(|l| l.unwrap()).collect::<Vec<_>>();
    part1(&lines);
    part2(&lines)
}
