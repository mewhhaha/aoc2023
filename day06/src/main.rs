use std::io;

fn calculate_number_of_ways((max_time, record): (i64, i64)) -> i64 {
    // This is just solving for x in "x * (max_time - x) = record"

    let lower_bound =
        (0.5 * (max_time as f64 - ((4 * -record + max_time.pow(2)) as f64).sqrt())) + 1.0;
    let upper_bound =
        (0.5 * (max_time as f64 + ((4 * -record + max_time.pow(2)) as f64).sqrt())) - 1.0;

    return ((upper_bound.ceil() + 1.0) - lower_bound.floor()) as i64;
}

fn part1(lines: &Vec<String>) {
    let [max_times, records] = lines
        .iter()
        .map(|line| {
            line.split_ascii_whitespace()
                .filter_map(|l| l.parse::<i64>().ok())
        })
        .collect::<Vec<_>>()
        .try_into()
        .expect("There to be two rows");

    let product = max_times
        .zip(records)
        .map(calculate_number_of_ways)
        .product::<i64>();

    println!("Part1: {}", product);
}

fn part2(lines: &Vec<String>) {
    let [max_time, record] = lines
        .iter()
        .filter_map(|line| {
            line.chars()
                .filter(|c| c.is_digit(10))
                .collect::<String>()
                .parse::<i64>()
                .ok()
        })
        .collect::<Vec<_>>()
        .try_into()
        .expect("There to be two rows");

    let number_of_ways_to_beat = calculate_number_of_ways((max_time, record));

    println!("Part2: {}", number_of_ways_to_beat);
}

fn main() {
    let lines = io::stdin().lines().map(|l| l.unwrap()).collect::<Vec<_>>();
    part1(&lines);
    part2(&lines)
}
