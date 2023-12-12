use std::collections::HashMap;

use itertools::Itertools;

advent_of_code::solution!(12);

pub fn part_one(input: &str) -> Option<i64> {
    solve(input, true)
}

pub fn part_two(input: &str) -> Option<i64> {
    solve(input, false)
}

fn solve(input: &str, is_part_1: bool) -> Option<i64> {
    Some(
        parse_input(input, is_part_1)
            .iter()
            .map(|(springs, groups)| {
                let mut memo: HashMap<(usize, usize), i64> = Default::default();
                arrangement_count(0, 0, springs, groups, &mut memo)
            })
            .sum(),
    )
}

fn parse_input(input: &str, part1: bool) -> Vec<(String, Vec<usize>)> {
    input
        .split('\n')
        .filter(|l| !l.is_empty())
        .map(|l| {
            let (springs, group_str) = l.split_once(' ').unwrap();
            let groups = group_str
                .split(',')
                .map(|v| v.parse().unwrap())
                .collect::<Vec<usize>>();

            if part1 {
                (format!("{}.", springs), groups)
            } else {
                let s_repeat = format!("{}?", springs).repeat(5);
                let expanded_springs = &s_repeat[..s_repeat.len() - 1];
                let exapnded_groups = groups
                    .iter()
                    .cycle()
                    .take(groups.len() * 5)
                    .copied()
                    .collect_vec();

                (format!("{}.", expanded_springs), exapnded_groups)
            }
        })
        .collect_vec()
}

fn arrangement_count(
    p: usize,
    g: usize,
    springs: &String,
    groups: &[usize],
    memo: &mut HashMap<(usize, usize), i64>,
) -> i64 {
    // Check memo cache first - 🏎️
    if let Some(cached) = memo.get(&(p, g)) {
        return *cached;
    }

    // No more groups
    if g >= groups.len() {
        // But there are still springs left in the row - not a solution
        if p < springs.len() && springs[p..].contains('#') {
            return 0;
        }
        return 1; // Solution found! 🎉
    }

    // No more springs, but all groups haven't been arranged - not a solution
    if p >= springs.len() {
        return 0;
    }

    let cur_spring = springs.chars().nth(p).unwrap();

    let group_size = groups[g];
    let group_end = (p + group_size).min(springs.len());
    let group_neighbor = springs
        .chars()
        .nth(p + group_size)
        .unwrap_or(springs.chars().last().unwrap());

    let count = match cur_spring {
        '?' => {
            if !springs[p..group_end].contains('.') && group_neighbor != '#' {
                arrangement_count(p + group_size + 1, g + 1, springs, groups, memo)
                    + arrangement_count(p + 1, g, springs, groups, memo)
            } else {
                // Working spring, move on to check the next one
                arrangement_count(p + 1, g, springs, groups, memo)
            }
        }
        '#' => {
            // Check if damaged group can start here, from this existing damaged spring
            if !springs[p..group_end].contains('.') && group_neighbor != '#' {
                arrangement_count(p + group_size + 1, g + 1, springs, groups, memo)
            } else {
                0 // Not a solution
            }
        }
        // Working spring, move on to check the next one
        '.' => arrangement_count(p + 1, g, springs, groups, memo),
        _ => unreachable!("Should never happen!"),
    };

    memo.insert((p, g), count);
    count
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(21));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(525152));
    }
}
