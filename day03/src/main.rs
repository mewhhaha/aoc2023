use std::io::{self, Read};

fn part1(buffer: &Vec<u8>) {
    let width = 1 + buffer
        .iter()
        .position(|c| *c as char == '\n')
        .expect("At least one row") as i32;

    let adjacent = |i: usize| {
        let j = i as i32;

        let kernel: [i32; 9] = [
            j - width - 1,
            j - width,
            j - width + 1,
            j - 1,
            j,
            j + 1,
            j + width - 1,
            j + width,
            j + width + 1,
        ];

        kernel.map(|j| if j < 0 { None } else { buffer.get(j as usize) })
    };

    let is_symbol = |u: &u8| {
        let c = *u as char;
        match c {
            '.' | '\n' => false,
            _ => !u.is_ascii_digit(),
        }
    };

    let mut sum = 0;
    let mut number = "".to_string();
    let mut valid = false;
    for (i, v) in buffer.iter().enumerate() {
        let c = *v as char;

        let is_digit = c.is_ascii_digit();
        if is_digit {
            number.push(c);

            if !valid {
                valid = adjacent(i)
                    .iter()
                    .any(|v| v.map(is_symbol).unwrap_or(false));
            }

            continue;
        }

        if valid && number != "" {
            sum += number.parse::<i32>().unwrap();
        }

        valid = false;
        number = "".to_string();
    }

    println!("Part1: {}", sum);
}

fn main() {
    let mut buffer = vec![];
    let _ = io::stdin().read_to_end(&mut buffer);

    // SON OF A BITCH WINDOWS \r\n
    let fixed_buffer = buffer.into_iter().filter(|c| *c != '\r' as u8).collect();

    part1(&fixed_buffer);
}
