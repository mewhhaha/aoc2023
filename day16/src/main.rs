use std::{collections::HashSet, io, ops::Add};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct V2<T>(T, T);

impl<T: Add<Output = T>> Add for V2<T> {
    type Output = V2<T>;

    fn add(self, rhs: Self) -> Self::Output {
        V2(self.0 + rhs.0, self.1 + rhs.1)
    }
}

fn rotate(v: V2<i32>, deg: i32) -> V2<i32> {
    match deg {
        90 => V2(-v.1, v.0),
        180 => V2(-v.0, -v.1),
        270 => V2(v.1, -v.0),
        -90 => V2(v.1, -v.0),
        -180 => V2(-v.0, -v.1),
        -270 => V2(-v.1, v.0),
        _ => panic!("Invalid rotation"),
    }
}

fn part1(lines: &Vec<String>) {
    let grid = lines
        .into_iter()
        .map(|l| l.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let get_cell = |x: i32, y: i32| {
        if x < 0 || y < 0 {
            return None;
        }
        grid.get(y as usize).and_then(|r| r.get(x as usize))
    };

    let mut set = HashSet::new();

    let mut memory = HashSet::new();

    let start = (V2(-1, 0), V2(1, 0));
    let mut rays = vec![start];
    while let Some((mut pos, vel)) = rays.pop() {
        // We keep a memory of positions and velocity so that if we encounter the same (ie: a loop) we'll quit prematurely
        if memory.contains(&(pos, vel)) {
            continue;
        }

        memory.insert((pos, vel));
        set.insert(pos);
        pos = pos + vel;

        match get_cell(pos.0, pos.1) {
            Some('|') if vel.0 != 0 => {
                rays.push((pos, V2(0, 1)));
                rays.push((pos, V2(0, -1)));
            }
            Some('-') if vel.1 != 0 => {
                rays.push((pos, V2(1, 0)));
                rays.push((pos, V2(-1, 0)));
            }
            Some('/') => {
                let rotation = if vel.0 == 0 { 90 } else { -90 };
                rays.push((pos, rotate(vel, rotation)));
            }
            Some('\\') => {
                let rotation = if vel.0 != 0 { 90 } else { -90 };
                rays.push((pos, rotate(vel, rotation)));
            }
            Some(_) => rays.push((pos, vel)),
            _ => (),
        }
    }

    // We remove the starting point since it's outside the grid
    set.remove(&start.0);

    println!("Part1: {}", set.len());
}

fn part2(lines: &Vec<String>) {
    println!("Part2: {}", "");
}

fn main() {
    let lines = io::stdin().lines().map(|l| l.unwrap()).collect::<Vec<_>>();
    part1(&lines);
    part2(&lines);
}
