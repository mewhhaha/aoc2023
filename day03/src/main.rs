#![feature(slice_group_by)]
use std::io::{self, Read};

fn adjacent_positions(i: usize, width: i32) -> [Option<usize>; 8] {
    let j = i as i32;

    let kernel: [i32; 8] = [
        j - width - 1,
        j - width,
        j - width + 1,
        j - 1,
        j + 1,
        j + width - 1,
        j + width,
        j + width + 1,
    ];

    kernel.map(|j| if j < 0 { None } else { Some(j as usize) })
}

fn get_width(buffer: &Vec<u8>) -> i32 {
    1 + buffer
        .iter()
        .position(|c| *c as char == '\n')
        .expect("At least one row") as i32
}

fn part1(buffer: &Vec<u8>) {
    let width = get_width(buffer);

    let any_adjacent_symbol = |i: usize| {
        fn is_symbol(u: &u8) -> bool {
            let c = *u as char;
            match c {
                '.' | '\n' => false,
                _ => !u.is_ascii_digit(),
            }
        }

        let is_adjacent_symbol = |j: Option<usize>| {
            j.and_then(|j| buffer.get(j).map(is_symbol))
                .unwrap_or(false)
        };

        adjacent_positions(i, width)
            .into_iter()
            .any(is_adjacent_symbol)
    };

    let mut sum = 0;
    let mut number = "".to_string();
    let mut valid = false;
    for (i, v) in buffer.iter().enumerate() {
        let c = *v as char;

        let is_digit = c.is_ascii_digit();
        if is_digit {
            number.push(c);

            if !valid {
                valid = any_adjacent_symbol(i)
            }

            continue;
        }

        if valid && number != "" {
            sum += number.parse::<i32>().unwrap();
        }

        valid = false;
        number = "".to_string();
    }

    println!("Part1: {}", sum);
}

fn part2(buffer: &Vec<u8>) {
    let width = get_width(buffer);

    let adjacent_gears = |i: usize| {
        let is_adjacent_gear = |j: Option<usize>| {
            j.and_then(|j| {
                buffer
                    .get(j)
                    .and_then(|u| if *u == '*' as u8 { Some(j) } else { None })
            })
        };
        adjacent_positions(i, width)
            .into_iter()
            .filter_map(is_adjacent_gear)
    };

    let mut gears = vec![];

    let mut sum = vec![];
    let mut number = "".to_string();
    for (i, v) in buffer.iter().enumerate() {
        let c = *v as char;

        let is_digit = c.is_ascii_digit();
        if is_digit {
            number.push(c);

            for gear in adjacent_gears(i) {
                if !sum.contains(&gear) {
                    sum.push(gear);
                }
            }

            continue;
        }

        for gear in sum {
            gears.push((gear, number.parse::<i32>().unwrap()));
        }

        sum = vec![];
        number = "".to_string();
    }

    gears.sort_by(|a, b| a.0.cmp(&b.0));

    let s: i32 = gears
        .group_by(|a, b| a.0 == b.0)
        .filter(|gs| gs.len() == 2)
        .map(|gs| gs.into_iter().map(|g| g.1).product::<i32>())
        .sum();

    println!("Part2: {}", s);
}

fn main() {
    let mut buffer = vec![];
    let _ = io::stdin().read_to_end(&mut buffer);

    // SON OF A BITCH WINDOWS \r\n
    let fixed_buffer = buffer.into_iter().filter(|c| *c != '\r' as u8).collect();

    part1(&fixed_buffer);
    part2(&fixed_buffer);
}
