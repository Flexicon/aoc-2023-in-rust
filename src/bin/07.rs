use std::cmp::Ordering;

use itertools::Itertools;

advent_of_code::solution!(7);

const LABELS: &str = "23456789TJQKA";
const LABELS_P2: &str = "J23456789TQKA";

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
enum HandKind {
    HighCard = 1,
    OnePair,
    TwoPair,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind,
}

#[derive(Debug, PartialEq, Eq)]
struct Hand {
    kind: HandKind,
    value: String,
    bid: u32,
    part: u8,
}

impl Hand {
    fn labels(&self) -> &str {
        if self.part == 1 {
            LABELS
        } else {
            LABELS_P2
        }
    }
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> Ordering {
        match self.kind.cmp(&other.kind) {
            Ordering::Equal => {
                for (a, b) in self.value.chars().zip(other.value.chars()) {
                    match cmp_labels(self.labels(), a, b) {
                        Ordering::Equal => continue,
                        ord => return ord,
                    }
                }
                Ordering::Equal
            }
            ord => ord,
        }
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    Some(solve(input, 1, parse_kind_p1))
}

pub fn part_two(input: &str) -> Option<u32> {
    Some(solve(input, 2, parse_kind_p2))
}

fn solve(input: &str, part: u8, kind_parse_fn: fn(&str) -> HandKind) -> u32 {
    input
        .split('\n')
        .filter(|l| !l.trim().is_empty())
        .map(|line| {
            let (hand, bid) = line.split_once(' ').unwrap();

            Hand {
                kind: kind_parse_fn(hand),
                value: String::from(hand),
                bid: bid.parse().unwrap(),
                part,
            }
        })
        .sorted()
        .enumerate()
        .map(|(i, h)| h.bid * (i as u32 + 1))
        .sum()
}

fn cmp_labels(lbls: &str, a: char, b: char) -> Ordering {
    lbls.find(a).unwrap().cmp(&lbls.find(b).unwrap())
}

fn parse_kind_p1(hand: &str) -> HandKind {
    let repeats = hand
        .chars()
        .sorted()
        .map(|c| (c, 1))
        .coalesce(|(a, n), (b, m)| {
            if a == b {
                Ok((a, n + m))
            } else {
                Err(((a, n), (b, m)))
            }
        })
        .filter_map(|(_, n)| if n > 1 { Some(n.to_string()) } else { None })
        .sorted()
        .collect::<Vec<String>>()
        .join("");

    repeated_amounts_as_kind(repeats.as_str())
}

fn parse_kind_p2(hand: &str) -> HandKind {
    let mut jokers = 0;
    let mut pairs = hand
        .chars()
        .sorted()
        .filter_map(|c| {
            if c == 'J' {
                jokers += 1;
                None
            } else {
                Some((c, 1))
            }
        })
        .coalesce(|(a, n), (b, m)| {
            if a == b {
                Ok((a, n + m))
            } else {
                Err(((a, n), (b, m)))
            }
        })
        .map(|(_, n)| n)
        .sorted()
        .collect::<Vec<i32>>();

    if pairs.is_empty() {
        pairs = Vec::from([jokers]);
    } else {
        let highest_pair = pairs.last_mut().unwrap();
        *highest_pair += jokers;
    }

    let repeats = pairs
        .iter()
        .filter(|v| v > &&1)
        .map(|v| v.to_string())
        .collect::<Vec<String>>()
        .join("");

    repeated_amounts_as_kind(repeats.as_str())
}

fn repeated_amounts_as_kind(repeats: &str) -> HandKind {
    match repeats {
        "2" => HandKind::OnePair,
        "3" => HandKind::ThreeOfAKind,
        "4" => HandKind::FourOfAKind,
        "5" => HandKind::FiveOfAKind,
        "23" => HandKind::FullHouse,
        "22" => HandKind::TwoPair,
        "" => HandKind::HighCard,
        v => panic!("Unexpected repeated cards value: '{v}'"),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(6440));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(5905));
    }
}
