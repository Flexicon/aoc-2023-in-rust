use itertools::{enumerate, Itertools};

advent_of_code::solution!(11);

pub fn part_one(input: &str) -> Option<u64> {
    Some(solve(input, 2))
}

pub fn part_two(input: &str) -> Option<u64> {
    Some(solve(input, 1_000_000))
}

fn solve(input: &str, expansion_size: usize) -> u64 {
    let grid = parse_grid(input);
    let (empty_rows, empty_cols) = find_empty_rows_and_cols(&grid);
    let galaxies = find_galaxies(&grid);

    let mut distances: Vec<u64> = Default::default();

    for (i, g1) in enumerate(&galaxies) {
        for g2 in galaxies.iter().skip(i) {
            let extra_cols = empty_cols
                .iter()
                .filter(|v| (g1.1..=g2.1).contains(v) || (g2.1..=g1.1).contains(v))
                .count()
                * (expansion_size - 1);
            let extra_rows = empty_rows
                .iter()
                .filter(|v| (g1.0..=g2.0).contains(v) || (g2.0..=g1.0).contains(v))
                .count()
                * (expansion_size - 1);
            let diff = (g1.0).abs_diff(g2.0) + (g1.1).abs_diff(g2.1) + extra_cols + extra_rows;

            distances.push(diff as u64);
        }
    }

    distances.iter().sum()
}

fn find_galaxies(grid: &[Vec<char>]) -> Vec<(usize, usize)> {
    let mut galaxies: Vec<(usize, usize)> = Default::default();

    for (i, row) in enumerate(grid) {
        for (j, col) in enumerate(row) {
            if col == &'#' {
                galaxies.push((i, j))
            }
        }
    }

    galaxies
}

fn find_empty_rows_and_cols(grid: &[Vec<char>]) -> (Vec<usize>, Vec<usize>) {
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

    (empty_rows, empty_cols)
}

fn parse_grid(input: &str) -> Vec<Vec<char>> {
    input
        .split('\n')
        .filter(|l| !l.is_empty())
        .map(|l| l.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>()
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
        assert_eq!(result, Some(82000210));
    }
}
