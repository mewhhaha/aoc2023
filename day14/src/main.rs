use std::io;

fn rotate_90(grid: Vec<Vec<char>>) -> Vec<Vec<char>> {
    let n = grid.len();
    let m = grid[0].len();
    let mut new_grid = vec![vec![' '; n]; m];

    for i in 0..n {
        for j in 0..m {
            new_grid[j][n - 1 - i] = grid[i][j];
        }
    }

    new_grid
}

#[test]
fn test_rotate_90() {
    let grid = vec![
        vec!['a', 'b', 'c'],
        vec!['d', 'e', 'f'],
        vec!['g', 'h', 'i'],
    ];
    let rotated = rotate_90(grid);

    assert_eq!(
        rotated,
        vec![
            vec!['g', 'd', 'a'],
            vec!['h', 'e', 'b'],
            vec!['i', 'f', 'c']
        ]
    );
}

#[test]
fn test_rotate_360() {
    let grid = vec![
        vec!['a', 'b', 'c'],
        vec!['d', 'e', 'f'],
        vec!['g', 'h', 'i'],
    ];
    let rotated = rotate_90(rotate_90(rotate_90(rotate_90(grid))));

    assert_eq!(
        rotated,
        vec![
            vec!['a', 'b', 'c'],
            vec!['d', 'e', 'f'],
            vec!['g', 'h', 'i'],
        ]
    );
}

fn tilt_north(mut grid: Vec<Vec<char>>) -> Vec<Vec<char>> {
    for y in 0..grid.len() {
        for x in 0..grid[0].len() {
            if grid[y][x] == 'O' {
                for j in (0..y).rev() {
                    if grid[j][x] == '.' {
                        grid[j][x] = 'O';
                        grid[j + 1][x] = '.';
                    } else {
                        break;
                    }
                }
            }
        }
    }

    grid
}

fn count_load(grid: &Vec<Vec<char>>) -> usize {
    let mut sum = 0;
    for y in 0..grid.len() {
        for x in 0..grid[0].len() {
            if grid[y][x] == 'O' {
                let load = grid.len() - y;
                sum += load;
            }
        }
    }

    sum
}

fn part1(lines: &Vec<String>) {
    let grid = lines
        .iter()
        .map(|s| s.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let tilted_grid = tilt_north(grid);

    let sum = count_load(&tilted_grid);

    println!("Part1: {}", sum);
}

fn part2(lines: &Vec<String>) {
    let mut grid = lines
        .iter()
        .map(|s| s.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let mut memory = vec![];
    'outer: for i in 0..1_000_000_000 {
        for _ in 0..4 {
            grid = tilt_north(grid);
            grid = rotate_90(grid);
        }

        let value = count_load(&grid);

        memory.push(value);

        for j in 2..memory.len() / 2 {
            let fst = memory.iter().rev().take(j);
            let snd = memory.iter().rev().skip(j).take(j);

            if fst.zip(snd).all(|c| c.0 == c.1) {
                let length = 1_000_000_000 - i;
                let iterations_left = length % j;

                let value = memory.iter().rev().nth((j + 1) - iterations_left).unwrap();

                println!("Part2: {:?}", value);
                break 'outer;
            }
        }
    }
}

fn main() {
    let lines = io::stdin().lines().map(|l| l.unwrap()).collect::<Vec<_>>();
    part1(&lines);
    part2(&lines);
}
