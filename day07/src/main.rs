#![feature(slice_split_once)]
#![feature(slice_group_by)]
use std::{cmp::Ordering, io};

#[derive(Debug, Clone, PartialEq, Eq)]
struct PokerHand(Vec<u32>, u64);

fn to_card_value(c: char) -> u32 {
    match c {
        'A' => 14,
        'K' => 13,
        'Q' => 12,
        'J' => 11,
        'T' => 10,
        _ => c.to_digit(10).expect("To be a digit"),
    }
}

fn to_hand_value(cards: &Vec<u32>) -> u32 {
    let mut sorted_cards = cards.clone();
    sorted_cards.sort_unstable();
    let mut grouped_cards = sorted_cards.group_by(|a, b| a == b).collect::<Vec<_>>();
    grouped_cards.sort_by(|a, b| b.len().cmp(&a.len()));

    match grouped_cards.as_slice() {
        [[_, _, _, _, _]] => 7,
        [[_, _, _, _], _] => 6,
        [[_, _, _], [_, _]] => 5,
        [[_, _, _], _, _] => 4,
        [[_, _], [_, _], _] => 3,
        [[_, _], _, _, _] => 2,
        _ => 1,
    }
}

impl PartialOrd for PokerHand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        let self_value = to_hand_value(&self.0);
        let other_value = to_hand_value(&other.0);
        if self_value != other_value {
            return self_value.partial_cmp(&other_value);
        }
        return self.0.iter().zip(other.0.iter()).find_map(|(a, b)| {
            if a != b {
                Some(a.cmp(b))
            } else {
                None
            }
        });
    }
}

impl Ord for PokerHand {
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(&other).unwrap_or(Ordering::Equal)
    }
}

fn part1(lines: &Vec<String>) {
    let mut hands = lines
        .into_iter()
        .map(|line| {
            line.split_once(|c| c == ' ')
                .map(|(a, b)| {
                    PokerHand(
                        a.chars().map(to_card_value).collect::<Vec<_>>(),
                        b.parse::<u64>().expect("There to be a bid"),
                    )
                })
                .expect("There to be a space")
        })
        .collect::<Vec<_>>();

    hands.sort();

    let value = hands
        .into_iter()
        .enumerate()
        .map(|(i, PokerHand(_, bid))| {
            let multiplier = i + 1;
            return bid * multiplier as u64;
        })
        .sum::<u64>();

    println!("Part1: {}", value);
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct PokerJokerHand(Vec<u32>, u64);

fn to_joker_card_value(c: char) -> u32 {
    match c {
        'A' => 14,
        'K' => 13,
        'Q' => 12,
        'J' => 1,
        'T' => 10,
        _ => c.to_digit(10).expect("To be a digit"),
    }
}

fn to_joker_hand_value(cards: &Vec<u32>) -> u32 {
    let mut sorted_cards = cards.clone();
    sorted_cards.sort();
    let mut grouped_cards = sorted_cards.group_by(|a, b| a == b).collect::<Vec<_>>();
    grouped_cards.sort_by(|a, b| {
        if a[0] == 1 {
            return Ordering::Less;
        }

        if b[0] == 1 {
            return Ordering::Less;
        }

        return b.len().cmp(&a.len());
    });

    match grouped_cards.as_slice() {
        [[_, _, _, _, _]] => 7,
        [[_, _, _, _], [1]] => 7,
        [[_, _, _], [1, 1]] => 7,
        [[_, _], [1, 1, 1]] => 7,
        [[_], [1, 1, 1, 1]] => 7,
        [[_, _, _, _], _] => 6,
        [[_, _, _], _, [1]] => 6,
        [[_, _], _, [1, 1]] => 6,
        [[_], _, [1, 1, 1]] => 6,
        [[_, _, _], [_, _]] => 5,
        [[_, _], [_, _], [1]] => 5,
        [[_, _, _], _, _] => 4,
        [[_, _], _, _, [1]] => 4,
        [[_], _, _, [1, 1]] => 4,
        [[_, _], [_, _], _] => 3,
        [[_, _], _, _, _] => 2,
        [_, _, _, _, [1]] => 2,
        _ => 1,
    }
}

impl PartialOrd for PokerJokerHand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        let self_value = to_joker_hand_value(&self.0);
        let other_value = to_joker_hand_value(&other.0);
        if self_value != other_value {
            return self_value.partial_cmp(&other_value);
        }
        return self.0.iter().zip(other.0.iter()).find_map(|(a, b)| {
            if a != b {
                Some(a.cmp(b))
            } else {
                None
            }
        });
    }
}

impl Ord for PokerJokerHand {
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(&other).unwrap_or(Ordering::Equal)
    }
}

fn part2(lines: &Vec<String>) {
    let mut hands = lines
        .into_iter()
        .map(|line| {
            line.split_once(|c| c == ' ')
                .map(|(a, b)| {
                    PokerJokerHand(
                        a.chars().map(to_joker_card_value).collect::<Vec<_>>(),
                        b.parse::<u64>().expect("There to be a bid"),
                    )
                })
                .expect("There to be a space")
        })
        .collect::<Vec<_>>();

    hands.sort();

    let value = hands
        .into_iter()
        .enumerate()
        .map(|(i, PokerJokerHand(_, bid))| {
            let multiplier = i + 1;
            return bid * multiplier as u64;
        })
        .sum::<u64>();

    println!("Part2: {}", value);
}

fn main() {
    let lines = io::stdin().lines().map(|l| l.unwrap()).collect::<Vec<_>>();
    part1(&lines);
    part2(&lines)
}
