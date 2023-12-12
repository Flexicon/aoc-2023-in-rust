use itertools::Itertools;

advent_of_code::solution!(12);

pub fn part_one(input: &str) -> Option<i64> {
    let lines = parse_input(input);
    let mut sum = 0;

    for (springs, groups) in &lines {
        sum += arrangement_count(0, 0, springs, groups);
    }

    Some(sum)
}

pub fn part_two(_: &str) -> Option<u32> {
    None
}

fn parse_input(input: &str) -> Vec<(String, Vec<usize>)> {
    input
        .split('\n')
        .filter(|l| !l.is_empty())
        .map(|l| {
            let (springs, group_str) = l.split_once(' ').unwrap();
            let groups = group_str
                .split(',')
                .map(|v| v.parse().unwrap())
                .collect::<Vec<usize>>();

            (format!("{}.", springs), groups)
        })
        .collect_vec()
}

fn arrangement_count(p: usize, g: usize, springs: &String, groups: &[usize]) -> i64 {
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

    match cur_spring {
        '?' => {
            if !springs[p..group_end].contains('.') && group_neighbor != '#' {
                arrangement_count(p + group_size + 1, g + 1, springs, groups)
                    + arrangement_count(p + 1, g, springs, groups)
            } else {
                // Working spring, move on to check the next one
                arrangement_count(p + 1, g, springs, groups)
            }
        }
        '#' => {
            // Check if damaged group can start here, from this existing damaged spring
            if !springs[p..group_end].contains('.') && group_neighbor != '#' {
                arrangement_count(p + group_size + 1, g + 1, springs, groups)
            } else {
                0 // Not a solution
            }
        }
        // Working spring, move on to check the next one
        '.' => arrangement_count(p + 1, g, springs, groups),
        _ => unreachable!("Should never happen!"),
    }
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
        assert_eq!(result, None);
    }
}
