use std::io;

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

fn get_galaxies(universe: &Vec<Vec<char>>) -> Vec<(usize, usize)> {
    universe
        .iter()
        .enumerate()
        .flat_map(|(y, row)| {
            row.iter()
                .enumerate()
                .filter_map(move |(x, c)| if *c == '#' { Some((x, y)) } else { None })
        })
        .collect::<Vec<_>>()
}

fn manhattan_distance(a: &(usize, usize), b: &(usize, usize)) -> i64 {
    (a.0 as i64 - b.0 as i64).abs() + (a.1 as i64 - b.1 as i64).abs()
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

    let galaxies = get_galaxies(&universe);

    let mut sum = 0;

    for g1 in 0..galaxies.len() {
        for g2 in g1..galaxies.len() {
            sum += manhattan_distance(&galaxies[g1], &galaxies[g2]);
        }
    }

    // We count each pair doubly ([a,b] and [b,a]), so this just divides by 2 to get the right sum
    println!("Part1: {}", sum);
}

fn part2(lines: &Vec<String>) {
    let mut universe = lines
        .iter()
        .map(|l| l.chars().collect())
        .collect::<Vec<Vec<_>>>();

    let non_expanded_galaxies = get_galaxies(&universe);

    let mut non_expanded_sum = 0;

    for g1 in 0..non_expanded_galaxies.len() {
        for g2 in g1..non_expanded_galaxies.len() {
            non_expanded_sum +=
                manhattan_distance(&non_expanded_galaxies[g1], &non_expanded_galaxies[g2])
        }
    }

    universe = expand_rows(&universe);
    universe = transpose(&universe);
    universe = expand_rows(&universe);
    universe = transpose(&universe);

    let galaxies = get_galaxies(&universe);

    let mut expanded_sum = 0;

    for g1 in 0..galaxies.len() {
        for g2 in g1..galaxies.len() {
            expanded_sum += manhattan_distance(&galaxies[g1], &galaxies[g2])
        }
    }

    let difference = expanded_sum - non_expanded_sum;

    let sum = non_expanded_sum + difference * (1_000_000 - 1);

    println!("Part2: {}", sum);
}

fn main() {
    let lines = io::stdin().lines().map(|l| l.unwrap()).collect::<Vec<_>>();
    part1(&lines);
    part2(&lines);
}
