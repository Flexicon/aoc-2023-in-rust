use std::collections::HashMap;

use itertools::enumerate;

advent_of_code::solution!(14);

#[derive(Debug, Clone, Copy)]
enum Dir {
    North,
    West,
    South,
    East,
}

pub fn part_one(input: &str) -> Option<u32> {
    let lines = input
        .split('\n')
        .filter(|l| !l.is_empty())
        .collect::<Vec<_>>();

    let mut sq_rocks: Vec<(usize, usize)> = Default::default();
    let mut rd_rocks: Vec<(usize, usize)> = Default::default();

    for (i, l) in enumerate(&lines) {
        for (j, c) in l.chars().enumerate() {
            match c {
                '#' => sq_rocks.push((i, j)),
                'O' => rd_rocks.push(max_tilt((i, j), &sq_rocks, &rd_rocks)),
                _ => continue,
            }
        }
    }

    Some(rd_rocks.iter().map(|(i, _)| (lines.len() - i) as u32).sum())
}

pub fn part_two(input: &str) -> Option<u32> {
    let lines = input
        .split('\n')
        .filter(|l| !l.is_empty())
        .collect::<Vec<_>>();
    let bounds = (lines.len(), lines.first()?.len());

    let mut rocks: HashMap<(usize, usize), char> = Default::default();

    // Get initial placements for rocks
    for (i, l) in enumerate(&lines) {
        for (j, c) in l.chars().enumerate() {
            match c {
                '#' | 'O' => {
                    rocks.insert((i, j), c);
                }
                _ => continue,
            }
        }
    }

    // Perorm tilt
    let max_cycles = 1_000_000_000; // THIS TAKES WAY TOO DAMN LONG
    for step in 0..max_cycles {
        let dir = dir_for_step(step);
        let pct = (step as f32 / max_cycles as f32) * 100.0;
        println!("STEP: [{}/{}] {:.0}%", step + 1, max_cycles, pct);

        match dir {
            Dir::North | Dir::West => {
                for i in 0..bounds.0 {
                    for j in 0..bounds.1 {
                        let cur = (i, j);
                        if !rocks.contains_key(&cur) {
                            continue;
                        }

                        let c = *rocks.get(&cur).unwrap();
                        if c == 'O' {
                            let tilted_pos = max_tilt_with_dir(cur, dir, bounds, &rocks);
                            if cur != tilted_pos {
                                rocks.insert(tilted_pos, c);
                                rocks.remove(&cur);
                            }
                        }
                    }
                }
            }
            Dir::South | Dir::East => {
                for i in (0..bounds.0).rev() {
                    for j in (0..bounds.1).rev() {
                        let cur = (i, j);
                        if !rocks.contains_key(&cur) {
                            continue;
                        }

                        let c = *rocks.get(&cur).unwrap();
                        if c == 'O' {
                            let tilted_pos = max_tilt_with_dir(cur, dir, bounds, &rocks);
                            if cur != tilted_pos {
                                rocks.insert(tilted_pos, c);
                                rocks.remove(&cur);
                            }
                        }
                    }
                }
            }
        }
    }

    let sum = rocks
        .iter()
        .filter(|(_, c)| *c == &'O')
        .map(|((i, _), _)| (lines.len() - i) as u32)
        .sum();
    Some(sum)
}

fn max_tilt(
    cur: (usize, usize),
    sq_rocks: &[(usize, usize)],
    rd_rocks: &[(usize, usize)],
) -> (usize, usize) {
    if cur.0 > 0
        && !sq_rocks.contains(&(cur.0 - 1, cur.1))
        && !rd_rocks.contains(&(cur.0 - 1, cur.1))
    {
        max_tilt((cur.0 - 1, cur.1), sq_rocks, rd_rocks)
    } else {
        cur
    }
}

fn max_tilt_with_dir(
    cur: (usize, usize),
    dir: Dir,
    bounds: (usize, usize),
    rocks: &HashMap<(usize, usize), char>,
) -> (usize, usize) {
    match dir {
        Dir::North => {
            if cur.0 > 0 && !rocks.contains_key(&(cur.0 - 1, cur.1)) {
                max_tilt_with_dir((cur.0 - 1, cur.1), dir, bounds, rocks)
            } else {
                cur
            }
        }
        Dir::West => {
            if cur.1 > 0 && !rocks.contains_key(&(cur.0, cur.1 - 1)) {
                max_tilt_with_dir((cur.0, cur.1 - 1), dir, bounds, rocks)
            } else {
                cur
            }
        }
        Dir::South => {
            if cur.0 < bounds.0 - 1 && !rocks.contains_key(&(cur.0 + 1, cur.1)) {
                max_tilt_with_dir((cur.0 + 1, cur.1), dir, bounds, rocks)
            } else {
                cur
            }
        }
        Dir::East => {
            if cur.1 < bounds.1 - 1 && !rocks.contains_key(&(cur.0, cur.1 + 1)) {
                max_tilt_with_dir((cur.0, cur.1 + 1), dir, bounds, rocks)
            } else {
                cur
            }
        }
    }
}

fn dir_for_step(step: u32) -> Dir {
    match step % 4 {
        0 => Dir::North,
        1 => Dir::West,
        2 => Dir::South,
        3 => Dir::East,
        _ => unreachable!(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(136));
    }

    #[test]
    fn test_part_two() {
        // let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        // assert_eq!(result, Some(64));
    }
}
