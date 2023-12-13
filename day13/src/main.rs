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

fn find_row_reflection(grid: &Vec<Vec<char>>, smudges: usize) -> Option<usize> {
    for i in 1..grid.len() {
        let first_half = grid.iter().take(i).rev();
        let second_half = grid.iter().skip(i);

        let mismatches = first_half
            .zip(second_half)
            .map(|(a, b)| a.iter().zip(b.iter()).filter(|(a, b)| a != b).count())
            .sum::<usize>();

        if mismatches == smudges {
            return Some(i);
        }
    }

    None
}

fn parse_grids(lines: &Vec<String>) -> Vec<Vec<Vec<char>>> {
    lines
        .split(|s| s.is_empty())
        .map(|g| {
            g.into_iter()
                .map(|s| s.chars().collect::<Vec<_>>())
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>()
}

fn find_grid_reflection(grid: &Vec<Vec<char>>, smudges: usize) -> Option<usize> {
    let vertical_reflection = find_row_reflection(grid, smudges);
    if let Some(i) = vertical_reflection {
        return Some(i * 100);
    }

    let horizontal_reflection = find_row_reflection(&transpose(grid), smudges);
    if let Some(i) = horizontal_reflection {
        return Some(i);
    }

    return None;
}

fn part1(lines: &Vec<String>) {
    let grids = parse_grids(lines);

    let mut sum = 0;

    for grid in grids {
        let reflection = find_grid_reflection(&grid, 0);
        if let Some(i) = reflection {
            sum += i;
        }
    }

    println!("Part1: {}", sum);
}

fn part2(lines: &Vec<String>) {
    let grids = parse_grids(lines);

    let mut sum = 0;

    for grid in grids {
        let reflection = find_grid_reflection(&grid, 1);
        if let Some(i) = reflection {
            sum += i;
        }
    }

    println!("Part2: {}", sum);
}

fn main() {
    let lines = io::stdin().lines().map(|l| l.unwrap()).collect::<Vec<_>>();
    part1(&lines);
    part2(&lines);
}
