advent_of_code::solution!(9);

pub fn part_one(input: &str) -> Option<i32> {
    Some(solve(input, predict_p1))
}

pub fn part_two(input: &str) -> Option<i32> {
    Some(solve(input, predict_p2))
}

fn solve(input: &str, predict: fn(&Vec<i32>) -> i32) -> i32 {
    parse_sequences(input).iter().map(|seq| predict(&seq)).sum()
}

fn parse_sequences(input: &str) -> Vec<Vec<i32>> {
    input
        .split("\n")
        .filter(|l| !l.trim().is_empty())
        .map(|l| {
            l.split(" ")
                .map(|v| v.parse().unwrap())
                .collect::<Vec<i32>>()
        })
        .collect::<Vec<_>>()
}

fn predict_p1(seq: &Vec<i32>) -> i32 {
    let (_, lasts) = firsts_and_lasts(&seq);
    lasts.iter().sum()
}

fn predict_p2(seq: &Vec<i32>) -> i32 {
    let mut firsts = firsts_and_lasts(&seq).0.clone();
    firsts.reverse();

    firsts.iter().skip(1).fold(firsts[0], |acc, n| n - acc)
}

fn firsts_and_lasts(seq: &Vec<i32>) -> (Vec<i32>, Vec<i32>) {
    let mut firsts: Vec<i32> = Vec::from([*seq.first().unwrap()]);
    let mut lasts: Vec<i32> = Vec::from([*seq.last().unwrap()]);
    let mut cur = seq.clone();

    while !all_zeroes(&cur) {
        let next = cur
            .iter()
            .enumerate()
            .filter_map(|(i, n)| {
                if i < cur.len() - 1 {
                    let next_value = cur[i + 1];
                    Some(next_value - n)
                } else {
                    None
                }
            })
            .collect::<Vec<_>>();

        firsts.push(*next.first().unwrap());
        lasts.push(*next.last().unwrap());
        cur = next.clone();
    }

    (firsts, lasts)
}

fn all_zeroes(seq: &Vec<i32>) -> bool {
    seq.iter().all(|v| v == &0)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(114));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2));
    }
}
