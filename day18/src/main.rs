use core::panic;
use std::{collections::HashSet, io};

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
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

struct Instruction {
    direction: Direction,
    meters: i64,
}

#[derive(Eq, PartialEq, Clone, Copy, Debug, Hash)]
struct Point {
    x: i64,
    y: i64,
}

#[derive(Eq, PartialEq, Clone, Copy, Debug, Hash)]
struct Range {
    from: Point,
    to: Point,
}

fn parse_instruction(line: &String) -> Instruction {
    let [d, m, _]: [&str; 3] = line
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

    Instruction { direction, meters }
}

fn flood_outside_grid(grid: &Vec<Vec<Tile>>) -> Vec<Vec<Tile>> {
    let mut new_grid = grid.clone();
    fn get_cell<'a>(grid: &'a Vec<Vec<Tile>>, x: i32, y: i32) -> Option<&'a Tile> {
        if x < 0 || y < 0 {
            return None;
        }
        return grid.get(y as usize).and_then(|row| row.get(x as usize));
    }

    let top_edge = (0..new_grid[0].len()).map(|x| (x as i32, 0 as i32));
    let bottom_edge = (0..new_grid[0].len()).map(|x| (x as i32, new_grid.len() as i32 - 1));
    let left_edge = (0..new_grid.len()).map(|y| (0, y as i32));
    let right_edge = (0..new_grid.len()).map(|y| (new_grid[0].len() as i32 - 1, y as i32));

    let mut queue = top_edge
        .chain(bottom_edge)
        .chain(left_edge)
        .chain(right_edge)
        .filter(|position| get_cell(&new_grid, position.0, position.1) != Some(&Tile::Trench))
        .collect::<Vec<_>>();

    while let Some(position) = queue.pop() {
        new_grid[position.1 as usize][position.0 as usize] = Tile::Outside;

        let adjacent = [
            (position.0, position.1 - 1),
            (position.0, position.1 + 1),
            (position.0 - 1, position.1),
            (position.0 + 1, position.1),
        ];

        for adjacent in adjacent.iter() {
            if let Some(cell) = get_cell(&new_grid, adjacent.0, adjacent.1) {
                if *cell == Tile::Inside {
                    queue.push(*adjacent);
                }
            }
        }
    }

    new_grid
}

fn create_trench_grid(path: &HashSet<(i64, i64)>) -> Vec<Vec<Tile>> {
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

    grid
}

// For debugging
fn _print_grid(grid: &Vec<Vec<Tile>>) {
    for row in grid.iter() {
        for cell in row.iter() {
            print!(
                "{}",
                match cell {
                    Tile::Trench => '#',
                    Tile::Inside => '.',
                    Tile::Outside => ' ',
                }
            );
        }
        println!();
    }
}

fn dig_path(instructions: &Vec<Instruction>) -> HashSet<(i64, i64)> {
    let mut path = HashSet::new();
    let mut current = (0, 0);

    for Instruction {
        meters, direction, ..
    } in instructions.iter()
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

    path
}

fn dig_ranges(instructions: &Vec<Instruction>) -> Vec<Range> {
    let mut path = vec![];
    let mut current = Point { x: 0, y: 0 };

    for Instruction {
        meters, direction, ..
    } in instructions.iter()
    {
        let Point { x, y } = current;
        current = match direction {
            Direction::Up => Point { x, y: y - meters },
            Direction::Down => Point { x, y: y + meters },
            Direction::Right => Point { x: x + meters, y },
            Direction::Left => Point { x: x - meters, y },
        };

        path.push(Range {
            from: Point { x, y },
            to: current,
        });
    }

    path
}

fn part1(lines: &Vec<String>) {
    let instructions = lines.into_iter().map(parse_instruction).collect::<Vec<_>>();

    let path = dig_path(&instructions);

    let mut grid = create_trench_grid(&path);
    grid = flood_outside_grid(&grid);

    let sum = grid
        .into_iter()
        .flatten()
        .filter(|t| *t != Tile::Outside)
        .count();

    println!("Part1: {}", sum);
}

fn parse_other_instruction(line: &String) -> Instruction {
    let [_, _, c]: [&str; 3] = line
        .split(' ')
        .collect::<Vec<_>>()
        .try_into()
        .expect("There to be three columns");

    let color = c
        .get(2..c.len() - 1)
        .expect("There to be a color wrapped by parentheses");

    let (meters_chars, direction_chars) = color.split_at(5);

    let meters = i64::from_str_radix(meters_chars, 16).expect("There to be a hexadecimal number");
    let direction = match direction_chars {
        "0" => Direction::Right,
        "1" => Direction::Down,
        "2" => Direction::Left,
        "3" => Direction::Up,
        _ => panic!("Invalid direction"),
    };

    Instruction { direction, meters }
}

fn order_range(range: &Range) -> Range {
    if range.from.x > range.to.x || range.from.y > range.to.y {
        Range {
            from: range.to,
            to: range.from,
        }
    } else {
        range.clone()
    }
}

fn part2(lines: &Vec<String>) {
    let instructions = lines
        .into_iter()
        .map(parse_other_instruction)
        .collect::<Vec<_>>();

    let ranges = dig_ranges(&instructions);

    let mut y_ranges = vec![];
    let mut x_ranges = vec![];

    for range in ranges.iter() {
        if range.from.y == range.to.y {
            x_ranges.push(range.from.x);
        } else {
            y_ranges.push(range.from.y);
        }
    }

    x_ranges.sort();
    y_ranges.sort();

    let get_folded_x = |x: i64| {
        x_ranges
            .iter()
            .enumerate()
            .find(|(_, x2)| **x2 == x)
            .map(|(i, _)| i as i64 * 2)
    };

    let get_folded_y = |y: i64| {
        y_ranges
            .iter()
            .enumerate()
            .find(|(_, y2)| **y2 == y)
            .map(|(i, _)| i as i64 * 2)
    };

    let mut path = HashSet::new();
    for range in ranges.iter() {
        let ordered_range = order_range(range);
        let x_from = get_folded_x(ordered_range.from.x).unwrap();
        let y_from = get_folded_y(ordered_range.from.y).unwrap();

        let x_to = get_folded_x(ordered_range.to.x).unwrap();
        let y_to = get_folded_y(ordered_range.to.y).unwrap();

        for y in y_from..=y_to {
            for x in x_from..=x_to {
                path.insert((x, y));
            }
        }
    }

    let mut grid = create_trench_grid(&path);
    grid = flood_outside_grid(&grid);

    let mut sum = 0;
    for (y, row) in grid.iter().enumerate() {
        let y_multiplier = if y & 1 == 1 {
            let relative_y = y / 2;
            y_ranges[relative_y + 1] - y_ranges[relative_y] - 1
        } else {
            1
        };
        for (x, cell) in row.iter().enumerate() {
            let x_multiplier = if x & 1 == 1 {
                let relative_x = x / 2;
                x_ranges[relative_x + 1] - x_ranges[relative_x] - 1
            } else {
                1
            };
            sum += match cell {
                Tile::Outside => 0,
                _ => x_multiplier * y_multiplier,
            }
        }
    }

    println!("Part2: {}", sum);
}

fn main() {
    let lines = io::stdin().lines().map(|l| l.unwrap()).collect::<Vec<_>>();
    part1(&lines);
    part2(&lines);
}
