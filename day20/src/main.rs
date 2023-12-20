use std::{collections::VecDeque, io};

#[derive(Debug, Clone, Eq, PartialEq)]
enum Frequency {
    High,
    Low,
}

#[derive(Debug, Clone)]
struct Pulse {
    from: String,
    to: String,
    frequency: Frequency,
}

#[derive(Debug, Clone)]
enum Logic {
    None,
    FlipFlop(bool),
    Conjunction(Vec<Pulse>),
}

#[derive(Debug, Clone)]
struct Module {
    name: String,
    logic: Logic,
    output: Vec<String>,
}

fn parse_modules(lines: &Vec<String>) -> Vec<Module> {
    let mut modules = Vec::new();

    for line in lines {
        let (raw_input, raw_output) = line.split_once(" -> ").unwrap();

        let output = raw_output
            .split(", ")
            .map(str::to_string)
            .collect::<Vec<_>>();

        let logic = match raw_input.get(0..1) {
            Some("%") => Logic::FlipFlop(false),
            Some("&") => Logic::Conjunction(vec![]),
            _ => Logic::None,
        };

        let name = match logic {
            Logic::None => raw_input.to_string(),
            _ => raw_input.get(1..).unwrap().to_string(),
        };

        let module = Module {
            name: name.to_string(),
            logic,
            output,
        };

        modules.push(module);
    }

    let static_state = modules.clone();

    for module in modules.iter_mut() {
        if let Logic::Conjunction(inputs) = &mut module.logic {
            let ms = static_state
                .iter()
                .filter(|m| m.output.contains(&module.name))
                .map(|input| Pulse {
                    from: input.name.clone(),
                    to: module.name.clone(),
                    frequency: Frequency::Low,
                });

            inputs.extend(ms);
        }
    }

    modules
}

fn part1(lines: &Vec<String>) {
    let mut modules = parse_modules(lines);

    let mut low_pulses = 0;
    let mut high_pulses = 0;

    for _ in 0..1000 {
        let mut pulses = VecDeque::new();
        pulses.push_back(Pulse {
            from: "".to_string(),
            to: "broadcaster".to_string(),
            frequency: Frequency::Low,
        });

        while let Some(Pulse {
            from,
            to,
            frequency,
        }) = pulses.pop_front()
        {
            match frequency {
                Frequency::Low => low_pulses += 1,
                Frequency::High => high_pulses += 1,
            }

            if let Some(module) = modules.iter_mut().find(|m| m.name == to) {
                let mut new_frequency = frequency.clone();

                match &mut module.logic {
                    Logic::None => (),
                    Logic::FlipFlop(on) => {
                        if frequency == Frequency::High {
                            continue;
                        }

                        *on = !*on;

                        new_frequency = match *on {
                            true => Frequency::High,
                            false => Frequency::Low,
                        };
                    }
                    Logic::Conjunction(memory) => {
                        let index = memory.iter().position(|p| p.from == from).unwrap();
                        memory[index].frequency = frequency.clone();

                        new_frequency = if memory.iter().all(|p| p.frequency == Frequency::High) {
                            Frequency::Low
                        } else {
                            Frequency::High
                        };
                    }
                }

                for output in &module.output {
                    pulses.push_back(Pulse {
                        from: to.clone(),
                        to: output.clone(),
                        frequency: new_frequency.clone(),
                    });
                }
            }
        }
    }

    let sum = low_pulses * high_pulses;

    println!("Part1: {:?}", sum);
}

fn part2(lines: &Vec<String>) {
    println!("Part2: {}", "");
}

fn main() {
    let lines = io::stdin().lines().map(|l| l.unwrap()).collect::<Vec<_>>();
    part1(&lines);
    part2(&lines);
}
