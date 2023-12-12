use std::collections::HashMap;
use std::io;

struct Record {
    locations: Vec<char>,
    numbers: Vec<usize>,
}

fn parse_record(line: &String) -> Record {
    let (fst, snd) = line.split_once(' ').unwrap();

    return Record {
        locations: fst.chars().collect::<Vec<_>>(),
        numbers: snd
            .split(',')
            .map(|s| s.parse::<usize>().unwrap())
            .collect::<Vec<_>>(),
    };
}

fn count_arrangements(
    memo: &mut HashMap<String, u64>,
    locations: &[char],
    numbers: &[usize],
) -> u64 {
    let key = format!("{:?}:{:?}", locations, numbers);

    if let Some(result) = memo.get(&key) {
        return *result;
    }

    match numbers.first() {
        None if locations.contains(&'#') => return 0,
        None => return 1,
        Some(first) if locations.len() < *first => return 0,
        Some(first) => {
            let mut result: u64 = 0;

            if locations.len() > 0 && locations[0] != '#' {
                result += count_arrangements(memo, &locations[1..], numbers);
            }

            let (fst, snd) = locations.split_at(*first);
            let fit = fst.iter().all(|c| match c {
                '?' | '#' => true,
                _ => false,
            });

            if fit && snd.get(0) != Some(&'#') {
                let next_locations = if snd.len() == 0 { snd } else { &snd[1..] };
                result += count_arrangements(memo, next_locations, &numbers[1..]);
            }

            memo.insert(key, result);

            result
        }
    }
}

fn part1(lines: &Vec<String>) {
    let records = lines.into_iter().map(parse_record);

    let mut sum = 0;
    let mut memo = HashMap::new();
    for Record { locations, numbers } in records {
        let fits = count_arrangements(&mut memo, &locations, numbers.as_slice());
        sum += fits;
    }

    println!("Part1: {}", sum);
}

fn part2(lines: &Vec<String>) {
    let records = lines.into_iter().map(parse_record);

    let mut sum: u64 = 0;
    let mut memo = HashMap::new();

    for Record { locations, numbers } in records {
        let five_times_locations = locations
            .repeat(5)
            .chunks(locations.len())
            .into_iter()
            .collect::<Vec<_>>()
            .join(&'?');
        let five_times_numbers = numbers.repeat(5);
        let fits = count_arrangements(
            &mut memo,
            &five_times_locations,
            five_times_numbers.as_slice(),
        );

        sum += fits;
    }

    println!("Part2: {}", sum);
}

fn main() {
    let lines = io::stdin().lines().map(|l| l.unwrap()).collect::<Vec<_>>();
    part1(&lines);
    part2(&lines);
}
