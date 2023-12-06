use std::io;

fn part1(lines: &Vec<String>) {
    let [max_times, records] = lines
        .iter()
        .map(|line| {
            line.split_ascii_whitespace()
                .filter_map(|l| l.parse::<i32>().ok())
        })
        .collect::<Vec<_>>()
        .try_into()
        .expect("There to be two rows");

    let product = max_times
        .zip(records)
        .map(|(max_time, record)| {
            let lower_bound =
                (0.5 * (max_time as f32 - ((4 * -record + max_time.pow(2)) as f32).sqrt())) + 1.0;
            let upper_bound =
                (0.5 * (max_time as f32 + ((4 * -record + max_time.pow(2)) as f32).sqrt())) - 1.0;

            return (upper_bound.ceil() + 1.0) - lower_bound.floor();
        })
        .product::<f32>();

    println!("Part1: {}", product);
}

fn part2(lines: &Vec<String>) {
    let max_time = lines[0]
        .chars()
        .filter(|c| c.is_digit(10))
        .collect::<String>()
        .parse::<i64>()
        .expect("There to be a max time");

    let record = lines[1]
        .chars()
        .filter(|c| c.is_digit(10))
        .collect::<String>()
        .parse::<i64>()
        .expect("There to be a record");

    let lower_bound =
        (0.5 * (max_time as f64 - ((4 * -record + max_time.pow(2)) as f64).sqrt())) + 1.0;
    let upper_bound =
        (0.5 * (max_time as f64 + ((4 * -record + max_time.pow(2)) as f64).sqrt())) - 1.0;

    let number_of_ways_to_beat = (upper_bound.ceil() + 1.0) - lower_bound.floor();

    println!("Part1: {}", number_of_ways_to_beat);
}

fn main() {
    let lines = io::stdin().lines().map(|l| l.unwrap()).collect::<Vec<_>>();
    part1(&lines);
    part2(&lines)
}
