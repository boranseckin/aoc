#![allow(clippy::enum_variant_names)]

use std::{collections::HashMap, cmp::Ordering};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
enum Kind {
    // Five of a kind  = AAAAA
    FiveOfAKind = 6,
    // Four of a kind  = AA8AA
    FourOfAKind = 5,
    // Full house      = 23332
    FullHouse = 4,
    // Three of a kind = TTT98
    ThreeOfAKind = 3,
    // Two pair        = 23432
    TwoPair = 2,
    // One pair        = A23A4
    OnePair = 1,
    // High card       = 23456
    HighCard = 0,
}

impl Kind {
    fn new(string: &str) -> Self {
        let mut map = HashMap::new();

        string.chars().for_each(|c| {
            *map.entry(c).or_insert(0) += 1;
        });

        match map.keys().len() {
            1 => Self::FiveOfAKind,
            2 => {
                for value in map.values() {
                    if *value == 1 {
                        return Self::FourOfAKind
                    }
                }

                Self::FullHouse
            },
            3 => {
                for value in map.values() {
                    if *value == 3 {
                        return Self::ThreeOfAKind
                    }
                }

                Self::TwoPair
            },
            4 => Self::OnePair,
            5 => Self::HighCard,
            _ => unreachable!(),
        }
    }
}

#[derive(Debug, Clone)]
struct Hand {
    string: String,
    kind: Kind,
    bid: usize,
}

impl Hand {
    fn new(string: &str, bid: &str) -> Self {
        Hand {
            string: string.into(),
            kind: Kind::new(string),
            bid: bid.parse().unwrap(),
        }
    }
}

impl PartialEq for Hand {
    fn eq(&self, other: &Self) -> bool {
        self.kind == other.kind && self.string == other.string
    }
}

impl Eq for Hand {}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> Ordering {
        if let Some(ord) = self.kind.partial_cmp(&other.kind) {
            if ord != Ordering::Equal {
                return ord;
            } else {
                let order = HashMap::from([
                    ('A', 14), ('K', 13),
                    ('Q', 12), ('J', 11),
                    ('T', 10), ('9',  9),
                    ('8',  8), ('7',  7),
                    ('6',  6), ('5',  5),
                    ('4',  4), ('3',  3),
                    ('2',  2)
                ]);

                for (s, o) in self.string.chars().zip(other.string.chars()) {
                    if s != o {
                        return order.get(&s).unwrap().cmp(order.get(&o).unwrap());
                    }
                }
            }
        }

        Ordering::Equal
    }
}

fn main() {
    let input = include_str!("../../inputs/day7.input");

    let mut hands: Vec<Hand> = input
        .lines()
        .map(|h| {
            let (hand, bid) = h.split_once(' ').unwrap();
            Hand::new(hand, bid)
        })
        .collect();
    hands.sort();

    let sum: usize = hands
        .into_iter()
        .enumerate()
        .map(|(rank, hand)| { hand.bid * (rank + 1) })
        .sum();

    println!("{sum}");
}
