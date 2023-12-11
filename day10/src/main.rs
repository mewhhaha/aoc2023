#![feature(generic_arg_infer)]
#![feature(iter_intersperse)]
#![feature(let_chains)]
use std::{collections::BinaryHeap, io};

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

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Tile {
    Inside,
    Outside,
    Loop(char),
}

fn part2(lines: &Vec<String>) {
    let pipes = lines
        .iter()
        .map(|l| l.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let mut tiles = pipes
        .iter()
        .map(|l| vec![Tile::Inside; l.len()])
        .collect::<Vec<_>>();

    let (mut position, mut direction) = find_start(&pipes).unwrap();

    loop {
        position = move_in_direction(position, &direction);
        let pipe = &pipes[position.1][position.0];

        tiles[position.1][position.0] = Tile::Loop(*pipe);
        if *pipe == 'S' {
            break;
        }

        direction = follow_pipe(&direction, pipe).unwrap();
    }

    let length = tiles[0].len();
    tiles = tiles
        .into_iter()
        .intersperse(vec![Tile::Inside; length])
        .map(|l| l.into_iter().intersperse(Tile::Inside).collect::<Vec<_>>())
        .collect::<Vec<_>>();

    for y in 0..tiles.len() {
        for x in 0..tiles[0].len() {
            let get_tile = |(x, y): (usize, usize)| tiles.get(y).and_then(|l| l.get(x));
            let north = y.checked_sub(1).map(|cy| (x, cy)).and_then(get_tile);
            let south = get_tile((x, y + 1));
            let west = x.checked_sub(1).map(|cx| (cx, y)).and_then(get_tile);
            let east = get_tile((x + 1, y));

            match (north, south, west, east) {
                (
                    Some(Tile::Loop('|' | 'F' | '7' | 'S')),
                    Some(Tile::Loop('|' | 'L' | 'J' | 'S')),
                    _,
                    _,
                ) => {
                    tiles[y][x] = Tile::Loop('|');
                }
                (
                    _,
                    _,
                    Some(Tile::Loop('-' | 'F' | 'L' | 'S')),
                    Some(Tile::Loop('-' | 'J' | '7' | 'S')),
                ) => {
                    tiles[y][x] = Tile::Loop('-');
                }
                _ => (),
            }
        }
    }

    let top_edge = (0..tiles[0].len()).map(|x| (x, 0));
    let bottom_edge = (0..tiles[0].len()).map(|x| (x, tiles.len() - 1));
    let left_edge = (0..tiles.len()).map(|y| (0, y));
    let right_edge = (0..tiles.len()).map(|y| (tiles[0].len() - 1, y));

    let mut heap = top_edge
        .chain(bottom_edge)
        .chain(left_edge)
        .chain(right_edge)
        .filter(|(x, y)| match tiles[*y][*x] {
            Tile::Loop(_) => false,
            _ => true,
        })
        .collect::<BinaryHeap<_>>();

    while let Some((x, y)) = heap.pop() {
        let tile = tiles[y][x];

        if tile != Tile::Inside {
            continue;
        }

        tiles[y][x] = Tile::Outside;

        let get_tile = |(x, y): (usize, usize)| tiles.get(y).and_then(|l| l.get(x));

        let north = y.checked_sub(1).map(|cy| (x, cy));
        let south = Some((x, y + 1));
        let west = x.checked_sub(1).map(|cx| (cx, y));
        let east = Some((x + 1, y));

        for o in [north, south, west, east] {
            if let Some(pos) = o
                && get_tile(pos) == Some(&Tile::Inside)
            {
                heap.push(pos);
            }
        }
    }

    let tiles_inside = tiles
        .iter()
        .enumerate()
        .filter_map(|(y, l)| if y & 1 == 0 { Some(l) } else { None })
        .map(|l| {
            l.iter()
                .enumerate()
                .filter(|(x, t)| x & 1 == 0 && **t == Tile::Inside)
                .count()
        })
        .sum::<usize>();

    println!("Part2: {}", tiles_inside);
}

fn main() {
    let lines = io::stdin().lines().map(|l| l.unwrap()).collect::<Vec<_>>();
    part1(&lines);
    part2(&lines);
}
