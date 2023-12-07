use std::cmp::Ordering;

use itertools::Itertools;

advent_of_code::solution!(7);

const LABELS: &str = "23456789TJQKA";

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

#[derive(Debug, Eq)]
struct Hand {
    kind: HandKind,
    value: String,
    bid: u32,
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> Ordering {
        match self.kind.cmp(&other.kind) {
            Ordering::Equal => {
                for (a, b) in self.value.chars().zip(other.value.chars()) {
                    match cmp_labels(a, b) {
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

impl PartialEq for Hand {
    fn eq(&self, other: &Self) -> bool {
        self.cmp(other) == std::cmp::Ordering::Equal
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    Some(
        input
            .split("\n")
            .filter(|l| !l.trim().is_empty())
            .map(|line| {
                let (hand, bid) = line.split_once(" ").unwrap();

                Hand {
                    kind: parse_kind(hand),
                    value: String::from(hand),
                    bid: bid.parse().unwrap(),
                }
            })
            .sorted()
            .enumerate()
            .map(|(i, h)| h.bid * (i as u32 + 1))
            .sum(),
    )
}

pub fn part_two(_: &str) -> Option<u32> {
    None
}

fn cmp_labels(a: char, b: char) -> Ordering {
    LABELS.find(a).unwrap().cmp(&LABELS.find(b).unwrap())
}

fn parse_kind(hand: &str) -> HandKind {
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
        .collect::<Vec<String>>()
        .join("");

    match repeats.as_str() {
        "2" => HandKind::OnePair,
        "3" => HandKind::ThreeOfAKind,
        "4" => HandKind::FourOfAKind,
        "5" => HandKind::FiveOfAKind,
        "23" | "32" => HandKind::FullHouse,
        "22" => HandKind::TwoPair,
        _ => HandKind::HighCard,
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
    fn test_part_one_answer() {
        let result = part_one(&advent_of_code::template::read_file("inputs", DAY));
        assert_eq!(result, Some(248422077));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
