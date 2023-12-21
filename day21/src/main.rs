use std::{collections::HashSet, io, ops::Add};

fn part1(lines: &Vec<String>) {
    let mut grid = lines
        .into_iter()
        .map(|l| l.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let mut start = (0, 0);
    'done: for (y, row) in grid.iter_mut().enumerate() {
        for (x, c) in row.iter_mut().enumerate() {
            if *c == 'S' {
                start = (x as i32, y as i32);
                *c = '.';
                break 'done;
            }
        }
    }

    let get_cell = |x: i32, y: i32| {
        if x < 0 || y < 0 || y >= grid.len() as i32 || x >= grid[0].len() as i32 {
            return None;
        }
        return Some(grid[y as usize][x as usize]);
    };

    let mut queue = vec![start];
    for _ in 0..64 {
        let mut new_queue = vec![];
        while let Some((x, y)) = queue.pop() {
            if get_cell(x, y) != Some('.') {
                continue;
            }

            new_queue.push((x + 1, y));
            new_queue.push((x - 1, y));
            new_queue.push((x, y + 1));
            new_queue.push((x, y - 1));
        }

        new_queue.sort();
        new_queue.dedup();
        queue = new_queue;
    }

    let valid_spaces = queue
        .into_iter()
        .filter(|pos| get_cell(pos.0, pos.1) == Some('.'))
        .count();

    println!("Part1: {}", valid_spaces);
}

fn part2(lines: &Vec<String>) {
    println!("Part2: {}", "");
}

fn main() {
    let lines = io::stdin().lines().map(|l| l.unwrap()).collect::<Vec<_>>();
    part1(&lines);
    part2(&lines);
}
