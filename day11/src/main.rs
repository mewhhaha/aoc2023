use std::{collections::BinaryHeap, io};

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

#[test]
fn it_should_transpose() {
    let grid = vec![vec!['1', '3', '5'], vec!['2', '4', '6']];
    let transposed_grid = vec![vec!['1', '2'], vec!['3', '4'], vec!['5', '6']];
    assert_eq!(transpose(&grid), transposed_grid);
}

#[test]
fn it_should_be_isomorphic() {
    let grid = vec![vec!['1', '3', '5'], vec!['2', '4', '6']];
    assert_eq!(transpose(&transpose(&grid)), grid);
}

fn expand_rows(grid: &Vec<Vec<char>>) -> Vec<Vec<char>> {
    let mut new_grid = vec![];
    for chars in grid {
        let is_empty = chars.iter().all(|c| *c == '.');

        if is_empty {
            new_grid.push(chars.clone());
        }
        new_grid.push(chars.clone());
    }

    new_grid
}

fn print_universe(universe: &Vec<Vec<char>>) {
    for row in universe.iter() {
        for cell in row {
            print!("{}", cell);
        }
        println!();
    }
}

fn part1(lines: &Vec<String>) {
    let mut universe = lines
        .iter()
        .map(|l| l.chars().collect())
        .collect::<Vec<Vec<_>>>();

    universe = expand_rows(&universe);
    universe = transpose(&universe);
    universe = expand_rows(&universe);
    universe = transpose(&universe);

    let galaxies = universe
        .into_iter()
        .enumerate()
        .flat_map(|(y, row)| {
            row.into_iter()
                .enumerate()
                .filter_map(move |(x, c)| if c == '#' { Some((x, y)) } else { None })
        })
        .collect::<Vec<_>>();

    let mut sum = 0;

    for g1 in galaxies.iter() {
        for g2 in galaxies.iter() {
            let manhattan_distance =
                (g1.0 as i32 - g2.0 as i32).abs() + (g1.1 as i32 - g2.1 as i32).abs();

            sum += manhattan_distance;
        }
    }

    // We count each pair doubly ([a,b] and [b,a]), so this just divides by 2 to get the right sum
    println!("Part1: {}", sum / 2);
}

fn part2(lines: &Vec<String>) {
    println!("Part2: {}", "");
}

fn main() {
    let lines = io::stdin().lines().map(|l| l.unwrap()).collect::<Vec<_>>();
    part1(&lines);
    part2(&lines);
}
