use std::{
    collections::{BinaryHeap, HashSet},
    io,
};

#[derive(Eq, PartialEq, Debug)]
struct State {
    cost: u32,
    position: (i32, i32),
    direction: (i32, i32),
    counter: usize,
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

    let mut seen = HashSet::new();
    let mut queue = BinaryHeap::new();
    queue.push(State {
        cost: 0,
        position: (0, 0),
        counter: 3,
        direction: (0, 0),
    });

    let end = ((grid[0].len() - 1) as i32, (grid.len() - 1) as i32);
    let adjacent = [(0, 1), (0, -1), (1, 0), (-1, 0)];

    while let Some(st) = queue.pop() {
        if st.position == end {
            println!("Part1: {}", st.cost);
            break;
        }

        for new_direction in adjacent.iter() {
            let is_same_direction = st.direction == *new_direction;
            let new_position = add_position(&st.position, &new_direction);
            let new_counter = if is_same_direction { st.counter - 1 } else { 3 };

            let is_previous_position = new_position == sub_position(&st.position, &st.direction);
            if is_previous_position {
                continue;
            }

            if new_counter == 0 {
                continue;
            }

            if seen.contains(&(new_counter, new_direction, new_position)) {
                continue;
            }

            if let Some(c) = get_cell(new_position.0, new_position.1) {
                let new_cost = st.cost + c;
                seen.insert((new_counter, new_direction, new_position));

                queue.push(State {
                    cost: new_cost,
                    position: new_position,
                    counter: new_counter,
                    direction: *new_direction,
                });
            }
        }
    }
}

fn part2(lines: &Vec<String>) {
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

    let mut seen = HashSet::new();
    let mut queue = BinaryHeap::new();
    queue.push(State {
        cost: 0,
        position: (0, 0),
        counter: 0,
        direction: (1, 0),
    });

    queue.push(State {
        cost: 0,
        position: (0, 0),
        counter: 0,
        direction: (0, 1),
    });

    let end = ((grid[0].len() - 1) as i32, (grid.len() - 1) as i32);
    let adjacent = [(0, 1), (0, -1), (1, 0), (-1, 0)];

    let minimum_steps = 4;
    let maximum_steps = 10;

    while let Some(st) = queue.pop() {
        if st.position == end && st.counter >= minimum_steps {
            println!("Part2: {}", st.cost);
            break;
        }

        for new_direction in adjacent.iter() {
            let is_different_direction = st.direction != *new_direction;
            if is_different_direction && st.counter < minimum_steps {
                continue;
            }

            let new_counter = if is_different_direction {
                1
            } else {
                st.counter + 1
            };

            if new_counter > maximum_steps {
                continue;
            }

            let new_position = add_position(&st.position, &new_direction);

            if new_position == sub_position(&st.position, &st.direction) {
                continue;
            }

            if seen.contains(&(new_counter, new_direction, new_position)) {
                continue;
            }

            if let Some(c) = get_cell(new_position.0, new_position.1) {
                let new_cost = st.cost + c;
                seen.insert((new_counter, new_direction, new_position));

                queue.push(State {
                    cost: new_cost,
                    position: new_position,
                    counter: new_counter,
                    direction: *new_direction,
                });
            }
        }
    }
}

fn main() {
    let lines = io::stdin().lines().map(|l| l.unwrap()).collect::<Vec<_>>();
    part1(&lines);
    part2(&lines);
}
