use itertools::enumerate;

advent_of_code::solution!(10);

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum Dir {
    East = 1,
    South,
    West,
    North,
}

pub fn part_one(input: &str) -> Option<u32> {
    let (rows, start) = parse_rows_and_start(input);

    let initial = next_pipe(start, &rows, &None, false).unwrap();
    let mut prev_dir = Some(initial.0);
    let mut cur = initial.1;
    let mut steps = 1;

    while rows[cur.0][cur.1] != 'S' {
        let check_for_start = steps > 1;
        steps += 1;

        match next_pipe(cur, &rows, &prev_dir, check_for_start) {
            Some(v) => {
                prev_dir = Some(v.0);
                cur = v.1;
            }
            None => break,
        }
    }

    Some(steps / 2)
}

pub fn part_two(input: &str) -> Option<u32> {
    let (rows, start) = parse_rows_and_start(input);

    let initial = next_pipe(start, &rows, &None, false).unwrap();
    let mut prev_dir = Some(initial.0);
    let mut cur = initial.1;
    let mut steps = 1;
    let mut pipe_loop = Vec::from([start, cur]);

    while rows[cur.0][cur.1] != 'S' {
        let check_for_start = steps > 1;
        steps += 1;

        match next_pipe(cur, &rows, &prev_dir, check_for_start) {
            Some(v) => {
                prev_dir = Some(v.0);
                cur = v.1;
                pipe_loop.push(cur);
            }
            None => break,
        }
    }

    let mut inside_count = 0;
    for (i, row) in enumerate(&rows) {
        let mut inside = false;

        for (j, c) in enumerate(row) {
            let is_in_loop = pipe_loop.contains(&(i, j));

            if !is_in_loop && inside {
                inside_count += 1;
            } else if is_in_loop && matches!(c, '|' | 'J' | 'L') {
                // Only switch from inside to outside (and vice-versa) when
                // encountering the pipes connecting with a north cells.
                // Others simply cannot wrap any cell.
                inside = !inside;
            }
        }
    }

    Some(inside_count)
}

fn parse_rows_and_start(input: &str) -> (Vec<Vec<char>>, (usize, usize)) {
    let rows = input
        .split('\n')
        .filter(|l| !l.trim().is_empty())
        .map(|l| l.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let mut s_loc = (0, 0);
    for (i, r) in enumerate(rows.clone()) {
        for (j, c) in enumerate(r) {
            if c == 'S' {
                s_loc = (i, j);
                break;
            }
        }
    }

    (rows, (s_loc.0, s_loc.1))
}

fn next_pipe(
    cur: (usize, usize),
    rows: &[Vec<char>],
    prev: &Option<Dir>,
    check_for_start: bool,
) -> Option<(Dir, (usize, usize))> {
    let cur_pipe = rows[cur.0][cur.1];
    let to_check = &[
        (Dir::West, Dir::East),
        (Dir::North, Dir::South),
        (Dir::East, Dir::West),
        (Dir::South, Dir::North),
    ];

    for (from, to) in to_check {
        let next = check_dir_for_pipe(cur, rows, cur_pipe, prev, from, to, check_for_start);
        if let Some(next) = next {
            return Some((next.0, next.1));
        }
    }

    None
}

fn check_dir_for_pipe(
    cur: (usize, usize),
    rows: &[Vec<char>],
    cur_pipe: char,
    prev: &Option<Dir>,
    from: &Dir,
    to: &Dir,
    check_for_start: bool,
) -> Option<(Dir, (usize, usize))> {
    if (prev.is_none() || prev.as_ref().unwrap() != to)
        && can_connect(cur_pipe, to, true)
        && check_bounds(cur, rows, (from, to))
    {
        let next_loc = next_loc(cur, (from, to));
        let next_pipe = rows[next_loc.0][next_loc.1];

        if can_connect(next_pipe, from, check_for_start) {
            return Some((*from, next_loc));
        }
    }

    None
}

fn next_loc(cur: (usize, usize), moving: (&Dir, &Dir)) -> (usize, usize) {
    match moving {
        (Dir::West, Dir::East) => (cur.0, cur.1 + 1),
        (Dir::North, Dir::South) => (cur.0 + 1, cur.1),
        (Dir::East, Dir::West) => (cur.0, cur.1 - 1),
        (Dir::South, Dir::North) => (cur.0 - 1, cur.1),
        v => panic!("Should never happen - moving == {:?}", v),
    }
}

fn check_bounds(cur: (usize, usize), rows: &[Vec<char>], moving: (&Dir, &Dir)) -> bool {
    match moving {
        (Dir::West, Dir::East) => cur.1 < rows[cur.0].len() - 1,
        (Dir::North, Dir::South) => cur.0 < rows.len() - 1,
        (Dir::East, Dir::West) => cur.1 > 0,
        (Dir::South, Dir::North) => cur.0 > 0,
        v => panic!("Should never happen - moving == {:?}", v),
    }
}

fn can_connect(pipe: char, dir: &Dir, check_for_start: bool) -> bool {
    match pipe {
        'S' => check_for_start,
        '|' => matches!(dir, Dir::North | Dir::South),
        '-' => matches!(dir, Dir::East | Dir::West),
        'L' => matches!(dir, Dir::North | Dir::East),
        'J' => matches!(dir, Dir::North | Dir::West),
        '7' => matches!(dir, Dir::South | Dir::West),
        'F' => matches!(dir, Dir::South | Dir::East),
        _ => false,
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
        assert_eq!(result, Some(8));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file_part(
            "examples", DAY, 2,
        ));
        assert_eq!(result, Some(4));
    }

    #[test]
    fn test_part_two_complicated() {
        let result = part_two(&advent_of_code::template::read_file_part(
            "examples", DAY, 3,
        ));
        assert_eq!(result, Some(10));
    }
}
