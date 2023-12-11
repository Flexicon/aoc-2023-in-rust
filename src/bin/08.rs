use std::collections::HashMap;

advent_of_code::solution!(8);

#[derive(Debug, Clone, Copy)]
enum Dir {
    L,
    R,
}

#[derive(Debug, Clone, Copy)]
struct Path<'a> {
    l: &'a str,
    r: &'a str,
}

pub fn part_one(input: &str) -> Option<u32> {
    let (line1, rest) = input.split_once("\n\n").unwrap();
    let dirs: Vec<Dir> = line1
        .chars()
        .map(|c| match c {
            'R' => Dir::R,
            'L' => Dir::L,
            _ => panic!("Unexpected character in direction string: '{c}'"),
        })
        .collect();

    let map = rest
        .split('\n')
        .filter(|l| !l.trim().is_empty())
        .map(|l| {
            let (name, paths) = l.split_once(" = ").unwrap();
            (name, parse_paths(paths))
        })
        .collect::<HashMap<_, _>>();

    let mut cur = "AAA";
    let mut steps = 0;
    while cur != "ZZZ" {
        let dir = dirs[steps % dirs.len()];
        let paths = map[cur];
        steps += 1;

        cur = match dir {
            Dir::L => paths.l,
            Dir::R => paths.r,
        };
    }

    Some(steps as u32)
}

pub fn part_two(input: &str) -> Option<usize> {
    let (line1, rest) = input.split_once("\n\n").unwrap();
    let dirs: Vec<Dir> = line1
        .chars()
        .map(|c| match c {
            'R' => Dir::R,
            'L' => Dir::L,
            _ => panic!("Unexpected character in direction string: '{c}'"),
        })
        .collect();

    let map = rest
        .split('\n')
        .filter(|l| !l.trim().is_empty())
        .map(|l| {
            let (name, paths) = l.split_once(" = ").unwrap();
            (name, parse_paths(paths))
        })
        .collect::<HashMap<_, _>>();

    let steps: Vec<usize> = map
        .clone()
        .into_keys()
        .filter(|k| k.ends_with('A'))
        .map(|s| steps_to_any_end(s, &dirs, &map))
        .collect();

    // I have honestly no idea why LCM is the solution here, but it is 🤷‍♂️
    Some(steps.iter().fold(steps[0], |acc, s| lcm(acc, *s)))
}

fn parse_paths(val: &str) -> Path {
    let (l, r) = val
        .trim_matches(&['(', ')'] as &[_])
        .split_once(", ")
        .unwrap();
    Path { l, r }
}

fn steps_to_any_end(s: &str, dirs: &[Dir], map: &HashMap<&str, Path<'_>>) -> usize {
    let mut steps = 0;
    let mut cur = s;

    while !cur.ends_with('Z') {
        let dir = dirs[steps % dirs.len()];
        let paths = map[cur];
        steps += 1;

        cur = match dir {
            Dir::L => paths.l,
            Dir::R => paths.r,
        };
    }

    steps
}

fn lcm(first: usize, second: usize) -> usize {
    first * second / gcd(first, second)
}

fn gcd(first: usize, second: usize) -> usize {
    let mut max = first;
    let mut min = second;
    if min > max {
        std::mem::swap(&mut max, &mut min);
    }

    loop {
        let res = max % min;
        if res == 0 {
            return min;
        }

        max = min;
        min = res;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file_part(
            "examples", DAY, 1,
        ));
        assert_eq!(result, Some(6));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file_part(
            "examples", DAY, 2,
        ));
        assert_eq!(result, Some(6));
    }
}
