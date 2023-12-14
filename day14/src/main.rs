use std::io;

fn tilt_north(grid: &Vec<Vec<char>>) -> Vec<Vec<char>> {
    let mut new_grid = grid.clone();
    for y in 0..grid.len() {
        for x in 0..grid[0].len() {
            if grid[y][x] == 'O' {
                for j in (0..y).rev() {
                    if new_grid[j][x] == '.' {
                        new_grid[j][x] = 'O';
                        new_grid[j + 1][x] = '.';
                    } else {
                        break;
                    }
                }
            }
        }
    }

    new_grid
}

fn part1(lines: &Vec<String>) {
    let grid = lines
        .iter()
        .map(|s| s.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let tilted_grid = tilt_north(&grid);

    let mut sum = 0;
    for y in 0..tilted_grid.len() {
        for x in 0..tilted_grid[0].len() {
            if tilted_grid[y][x] == 'O' {
                let load = tilted_grid.len() - y;
                sum += load;
            }
        }
    }

    for line in tilted_grid {
        println!("{}", line.iter().collect::<String>());
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
