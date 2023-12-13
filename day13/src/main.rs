use std::io;

fn transpose(grid: &Vec<Vec<char>>) -> Vec<Vec<char>> {
    let mut new_grid = vec![];
    for x in 0..grid[0].len() {
        let mut row = vec![];
        for y in 0..grid.len() {
            row.push(grid[y][x]);
        }
        new_grid.push(row);
    }
    new_grid
}

fn find_vertical_reflection(grid: &Vec<Vec<char>>) -> Option<usize> {
    for i in 1..grid.len() {
        let first_half = grid.iter().take(i).rev();
        let second_half = grid.iter().skip(i);

        if first_half
            .zip(second_half)
            .all(|(a, b)| a.iter().zip(b.iter()).all(|(a, b)| a == b))
        {
            return Some(i);
        }
    }

    None
}

fn part1(lines: &Vec<String>) {
    let grids = lines
        .split(|s| s.is_empty())
        .map(|g| {
            g.into_iter()
                .map(|s| s.chars().collect::<Vec<_>>())
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    let mut sum = 0;

    for grid in grids {
        let vertical_reflection = find_vertical_reflection(&grid);
        if let Some(i) = vertical_reflection {
            sum += i * 100;
            continue;
        }

        let horizontal_reflection = find_vertical_reflection(&transpose(&grid));
        if let Some(i) = horizontal_reflection {
            sum += i;
            continue;
        }

        println!();
        for row in grid.iter() {
            println!("{}", row.iter().collect::<String>());
        }
    }

    println!("Part1: {}", sum);
}

fn part2(lines: &Vec<String>) {
    println!("Part2: {}", "");
}

fn main() {
    let lines = io::stdin().lines().map(|l| l.unwrap()).collect::<Vec<_>>();
    part1(&lines);
    part2(&lines);
}
