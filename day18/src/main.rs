use std::{collections::HashSet, io, ops::Add};

enum Direction {
    Up,
    Down,
    Right,
    Left,
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
enum Tile {
    Trench,
    Inside,
    Outside,
}

struct Instruction<'a> {
    direction: Direction,
    meters: i64,
    color: &'a str,
}

fn parse_instruction(line: &String) -> Instruction {
    let [d, m, c] = line
        .split(' ')
        .collect::<Vec<_>>()
        .try_into()
        .expect("There to be three columns");

    let direction = match d.chars().next().expect("There to be a first character") {
        'U' => Direction::Up,
        'D' => Direction::Down,
        'R' => Direction::Right,
        'L' => Direction::Left,
        _ => panic!("Invalid direction"),
    };

    let meters = m.parse::<i64>().expect("There to be a number");

    let color = c
        .get(1..c.len() - 1)
        .expect("There to be a color wrapped by parentheses");

    Instruction {
        direction,
        meters,
        color,
    }
}

// For debugging
// fn print_grid(grid: &Vec<Vec<Tile>>) {
//     for row in grid.iter() {
//         for cell in row.iter() {
//             print!(
//                 "{}",
//                 match cell {
//                     Tile::Trench => '#',
//                     Tile::Inside => '.',
//                     Tile::Outside => ' ',
//                 }
//             );
//         }
//         println!();
//     }
// }

fn part1(lines: &Vec<String>) {
    let instructions = lines.into_iter().map(parse_instruction).collect::<Vec<_>>();

    let mut path = HashSet::new();

    let mut current = (0, 0);
    for Instruction {
        meters, direction, ..
    } in instructions.into_iter()
    {
        let (x, y) = current;
        current = match direction {
            Direction::Up => (x, y - meters),
            Direction::Down => (x, y + meters),
            Direction::Right => (x + meters, y),
            Direction::Left => (x - meters, y),
        };

        let y_range = if current.1 > y {
            y..=current.1
        } else {
            current.1..=y
        };

        let x_range = if current.0 > x {
            x..=current.0
        } else {
            current.0..=x
        };

        for y in y_range {
            for x in x_range.clone() {
                path.insert((x, y));
            }
        }
    }

    println!("{}", path.len());

    let offset_x = *path.iter().map(|(x, _)| x).min().unwrap();
    let offset_y = *path.iter().map(|(_, y)| y).min().unwrap();
    let max_x = (*path.iter().map(|(x, _)| x).max().unwrap() - offset_x) as usize;
    let max_y = (*path.iter().map(|(_, y)| y).max().unwrap() - offset_y) as usize;

    let mut grid = vec![vec![Tile::Inside; max_x + 1]; max_y + 1];

    let trench = path
        .into_iter()
        .map(|(x, y)| ((x - offset_x) as usize, (y - offset_y) as usize))
        .collect::<Vec<_>>();

    for y in 0..grid.len() {
        for x in 0..grid[0].len() {
            if trench.contains(&(x, y)) {
                grid[y][x] = Tile::Trench
            }
        }
    }

    fn get_cell<'a>(grid: &'a Vec<Vec<Tile>>, x: i32, y: i32) -> Option<&'a Tile> {
        if x < 0 || y < 0 {
            return None;
        }
        return grid.get(y as usize).and_then(|row| row.get(x as usize));
    }

    let top_edge = (0..grid[0].len()).map(|x| (x as i32, 0 as i32));
    let bottom_edge = (0..grid[0].len()).map(|x| (x as i32, grid.len() as i32 - 1));
    let left_edge = (0..grid.len()).map(|y| (0, y as i32));
    let right_edge = (0..grid.len()).map(|y| (grid[0].len() as i32 - 1, y as i32));

    let mut queue = top_edge
        .chain(bottom_edge)
        .chain(left_edge)
        .chain(right_edge)
        .filter(|position| get_cell(&grid, position.0, position.1) != Some(&Tile::Trench))
        .collect::<Vec<_>>();

    while let Some(position) = queue.pop() {
        grid[position.1 as usize][position.0 as usize] = Tile::Outside;

        let adjacent = [
            (position.0, position.1 - 1),
            (position.0, position.1 + 1),
            (position.0 - 1, position.1),
            (position.0 + 1, position.1),
        ];

        for adjacent in adjacent.iter() {
            if let Some(cell) = get_cell(&grid, adjacent.0, adjacent.1) {
                if *cell == Tile::Inside {
                    queue.push(*adjacent);
                }
            }
        }
    }

    let sum = grid
        .into_iter()
        .flatten()
        .filter(|t| *t != Tile::Outside)
        .count();

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
