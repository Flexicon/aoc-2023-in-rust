use itertools::{enumerate, Itertools};

advent_of_code::solution!(11);

pub fn part_one(input: &str) -> Option<u32> {
    let grid = input
        .split('\n')
        .filter(|l| !l.is_empty())
        .map(|l| l.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let mut empty_cols: Vec<usize> = Default::default();
    let mut empty_rows: Vec<usize> = Default::default();

    for (i, _) in enumerate(&grid[0]) {
        let col_empty = !grid.iter().map(|row| row[i]).contains(&'#');

        if col_empty {
            empty_cols.push(i)
        }

        if grid[i].iter().all(|c| c == &'.') {
            empty_rows.push(i)
        }
    }

    let mut galaxies: Vec<(usize, usize)> = Default::default();
    for (i, row) in enumerate(&grid) {
        for (j, col) in enumerate(row) {
            if col == &'#' {
                galaxies.push((i, j))
            }
        }
    }

    let mut distances: Vec<u32> = Default::default();

    for (i, g1) in enumerate(&galaxies) {
        for g2 in galaxies.iter().skip(i) {
            if g1 == g2 {
                continue;
            }

            let extra_cols = empty_cols
                .iter()
                .filter(|v| (g1.1..=g2.1).contains(v) || (g2.1..=g1.1).contains(v))
                .count();
            let extra_rows = empty_rows
                .iter()
                .filter(|v| (g1.0..=g2.0).contains(v) || (g2.0..=g1.0).contains(v))
                .count();
            let diff = (g1.0).abs_diff(g2.0) + (g1.1).abs_diff(g2.1) + extra_cols + extra_rows;

            distances.push(diff as u32);
        }
    }

    Some(distances.iter().sum())
}

pub fn part_two(_: &str) -> Option<u32> {
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(374));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
