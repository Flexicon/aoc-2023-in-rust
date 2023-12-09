advent_of_code::solution!(9);

pub fn part_one(input: &str) -> Option<i32> {
    let sequences = input
        .split("\n")
        .filter(|l| !l.trim().is_empty())
        .map(|l| {
            l.split(" ")
                .map(|v| v.parse().unwrap())
                .collect::<Vec<i32>>()
        })
        .collect::<Vec<_>>();

    let mut predictions: Vec<i32> = Default::default();

    for seq in sequences {
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

            lasts.push(*next.last().unwrap());
            cur = next.clone();
        }

        predictions.push(lasts.iter().sum());
    }

    Some(predictions.iter().sum())
}

pub fn part_two(_: &str) -> Option<u32> {
    None
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
        assert_eq!(result, None);
    }
}
