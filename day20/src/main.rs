use std::{
    collections::{HashMap, VecDeque},
    io,
};

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
    FlipFlop(Vec<Pulse>, bool),
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
            Some("%") => Logic::FlipFlop(vec![], false),
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
        if let Logic::Conjunction(inputs) | Logic::FlipFlop(inputs, _) = &mut module.logic {
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

fn eval_pulse(
    module: &mut Module,
    pulses: &mut VecDeque<Pulse>,
    Pulse {
        from,
        to,
        frequency,
    }: Pulse,
) -> Option<Frequency> {
    let mut new_frequency = frequency.clone();

    match &mut module.logic {
        Logic::None => (),
        Logic::FlipFlop(_, on) => {
            if frequency == Frequency::High {
                return None;
            }

            *on = !*on;

            new_frequency = match *on {
                true => Frequency::High,
                false => Frequency::Low,
            };
        }
        Logic::Conjunction(inputs) => {
            let index = inputs.iter().position(|p| p.from == from).unwrap();
            inputs[index].frequency = frequency.clone();

            new_frequency = if inputs.iter().all(|p| p.frequency == Frequency::High) {
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

    return Some(new_frequency);
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

        while let Some(pulse) = pulses.pop_front() {
            match pulse.frequency {
                Frequency::Low => low_pulses += 1,
                Frequency::High => high_pulses += 1,
            }

            if let Some(module) = modules.iter_mut().find(|m| m.name == pulse.to) {
                eval_pulse(module, &mut pulses, pulse);
            }
        }
    }

    let sum = low_pulses * high_pulses;

    println!("Part1: {:?}", sum);
}

fn gcd(a: usize, b: usize) -> usize {
    let mut a = a;
    let mut b = b;

    while b != 0 {
        let t = b;
        b = a % b;
        a = t;
    }

    a
}

fn lcm(a: usize, b: usize) -> usize {
    (a * b) / gcd(a, b)
}

fn part2(lines: &Vec<String>) {
    let mut modules = parse_modules(lines);
    let mut history = HashMap::new();

    // This just searches for the first split in the circuit
    // This is likely the number we need to lcm since
    // The resulting number would be too big otherwise
    let mut dependencies = vec!["rx".to_string()];
    while let Some(dependency) = dependencies.pop() {
        let module = modules
            .iter()
            .find(|m| m.output.contains(&dependency))
            .unwrap();

        match &module.logic {
            Logic::None => panic!(),
            Logic::FlipFlop(_, _) => panic!(),
            Logic::Conjunction(inputs) => {
                dependencies.extend(inputs.iter().map(|p| p.from.clone()))
            }
        }

        if dependencies.len() > 1 {
            break;
        }
    }

    let mut press = 0;
    // Just testing out numbers until they all resolve the cn dependencies
    while history.len() != dependencies.len() {
        press += 1;

        let mut pulses = VecDeque::new();
        pulses.push_back(Pulse {
            from: "".to_string(),
            to: "broadcaster".to_string(),
            frequency: Frequency::Low,
        });

        while let Some(pulse) = pulses.pop_front() {
            if dependencies.contains(&pulse.from) && pulse.frequency == Frequency::High {
                let key = pulse.from.clone();
                if !history.contains_key(&key) {
                    history.insert(key, press);
                }
            }

            if let Some(module) = modules.iter_mut().find(|m| m.name == pulse.to) {
                eval_pulse(module, &mut pulses, pulse);
            }
        }
    }

    let mut multiple = None;
    for press in history.values() {
        if let Some(m) = multiple {
            multiple = Some(lcm(m, *press));
        } else {
            multiple = Some(*press);
        }
    }

    println!("Part2: {:?}", multiple.unwrap());
}

fn main() {
    let lines = io::stdin().lines().map(|l| l.unwrap()).collect::<Vec<_>>();
    part1(&lines);
    part2(&lines);
}
