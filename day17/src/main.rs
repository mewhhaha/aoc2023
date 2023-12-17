use std::{
    collections::{BinaryHeap, HashMap},
    io,
};

#[derive(Eq, PartialEq, Debug)]
struct State {
    cost: u32,
    position: (i32, i32),
    direction: (usize, (i32, i32)),
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        other.cost.partial_cmp(&self.cost)
    }
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other.cost.cmp(&self.cost)
    }
}

fn add_position(a: &(i32, i32), b: &(i32, i32)) -> (i32, i32) {
    (a.0 + b.0, a.1 + b.1)
}

fn sub_position(a: &(i32, i32), b: &(i32, i32)) -> (i32, i32) {
    (a.0 - b.0, a.1 - b.1)
}

fn part1(lines: &Vec<String>) {
    let grid = lines
        .into_iter()
        .map(|l| {
            l.chars()
                .map(|c| c.to_digit(10).unwrap())
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    let get_cell = |x: i32, y: i32| {
        if x < 0 || y < 0 {
            return None;
        }
        return grid.get(y as usize).and_then(|row| row.get(x as usize));
    };

    let mut seen = HashMap::new();
    let mut queue = BinaryHeap::new();
    queue.push(State {
        cost: 0,
        position: (0, 0),
        direction: (3, (0, 0)),
    });

    let end = ((grid[0].len() - 1) as i32, (grid.len() - 1) as i32);
    let adjacent = [(0, 1), (0, -1), (1, 0), (-1, 0)];

    while let Some(st) = queue.pop() {
        if st.position == end {
            println!("Part1: {}", st.cost);
            break;
        }

        for velocity in adjacent.iter() {
            let new_position = add_position(&st.position, &velocity);
            let new_countdown = if st.direction.1 == *velocity {
                st.direction.0 - 1
            } else {
                3
            };

            if new_position == sub_position(&st.position, &st.direction.1) {
                continue;
            }

            if new_countdown == 0 {
                continue;
            }

            if seen.contains_key(&(new_countdown, velocity, new_position)) {
                continue;
            }

            if let Some(c) = get_cell(new_position.0, new_position.1) {
                let new_cost = st.cost + c;
                seen.insert((new_countdown, velocity, new_position), new_cost);

                queue.push(State {
                    cost: new_cost,
                    position: new_position,
                    direction: (new_countdown, *velocity),
                });
            }
        }
    }
}

fn part2(lines: &Vec<String>) {
    println!("Part2: {}", "");
}

fn main() {
    let lines = io::stdin().lines().map(|l| l.unwrap()).collect::<Vec<_>>();
    part1(&lines);
    part2(&lines);
}
