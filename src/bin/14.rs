use itertools::enumerate;

advent_of_code::solution!(14);

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

pub fn part_two(_: &str) -> Option<u32> {
    None
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
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
