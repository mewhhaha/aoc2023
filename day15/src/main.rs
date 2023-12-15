use std::io;

fn part1(lines: &Vec<String>) {
    let steps = lines[0].split(',');

    let mut sum = 0;
    for step in steps {
        let mut value = 0;
        for c in step.chars() {
            let code = c as u32;
            value += code;
            value *= 17;
            value %= 256;
        }

        sum += value;
    }

    println!("Part1: {}", sum);
}

fn part2(lines: &Vec<String>) {
    println!("Part2: {:?}", "");
}

fn main() {
    let lines = io::stdin().lines().map(|l| l.unwrap()).collect::<Vec<_>>();
    part1(&lines);
    part2(&lines);
}
