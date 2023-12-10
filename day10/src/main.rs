#![feature(generic_arg_infer)]
use std::io;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Direction {
    North,
    East,
    South,
    West,
}

fn find_start(lines: &Vec<Vec<char>>) -> Option<((usize, usize), Direction)> {
    for (y, line) in lines.iter().enumerate() {
        for (x, c) in line.iter().enumerate() {
            if *c == 'S' {
                let north = y.checked_sub(1).map(|cy| lines[cy][x]);
                let south = lines.get(y + 1).map(|l| l[x]);
                let west = x.checked_sub(1).map(|cx| lines[y][cx]);
                let east = lines[y].get(x + 1);

                let direction = match (north, south, west, east) {
                    (Some('|' | 'F' | '7'), _, _, _) => Direction::North,
                    (_, Some('|' | 'L' | 'J'), _, _) => Direction::South,
                    (_, _, Some('-' | 'F' | 'L'), _) => Direction::West,
                    (_, _, _, Some('-' | '7' | 'J')) => Direction::East,
                    _ => panic!("Invalid start"),
                };

                return Some(((x, y), direction));
            }
        }
    }

    return None;
}

fn move_in_direction(position: (usize, usize), direction: &Direction) -> (usize, usize) {
    match direction {
        Direction::North => (position.0, position.1 - 1),
        Direction::East => (position.0 + 1, position.1),
        Direction::South => (position.0, position.1 + 1),
        Direction::West => (position.0 - 1, position.1),
    }
}

fn follow_pipe(direction: &Direction, pipe: &char) -> Option<Direction> {
    match (direction, pipe) {
        (dir, '|' | '-') => Some(*dir),
        (Direction::North, 'F') => Some(Direction::East),
        (Direction::North, '7') => Some(Direction::West),
        (Direction::South, 'L') => Some(Direction::East),
        (Direction::South, 'J') => Some(Direction::West),
        (Direction::East, 'J') => Some(Direction::North),
        (Direction::East, '7') => Some(Direction::South),
        (Direction::West, 'L') => Some(Direction::North),
        (Direction::West, 'F') => Some(Direction::South),
        _ => None,
    }
}

fn part1(lines: &Vec<String>) {
    let pipes = lines
        .iter()
        .map(|l| l.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let (mut position, mut direction) = find_start(&pipes).unwrap();
    let mut moves = 0;

    loop {
        position = move_in_direction(position, &direction);
        let pipe = &pipes[position.1][position.0];
        moves += 1;

        if *pipe == 'S' {
            break;
        }

        direction = follow_pipe(&direction, pipe).unwrap();
    }

    let furthest_away = moves / 2;
    println!("Part1: {}", furthest_away);
}

fn part2(lines: &Vec<String>) {
    println!("Part2: {}", "");
}

fn main() {
    let lines = io::stdin().lines().map(|l| l.unwrap()).collect::<Vec<_>>();
    part1(&lines);
    part2(&lines);
}
