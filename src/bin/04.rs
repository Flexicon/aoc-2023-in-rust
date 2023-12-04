use std::collections::HashMap;

advent_of_code::solution!(4);

pub fn part_one(input: &str) -> Option<u32> {
    Some(
        input
            .lines()
            .map(|l| {
                let (raw_wins, raw_picks) =
                    l.split_once(": ").unwrap().1.split_once(" | ").unwrap();
                let (wins, picks) = (parse_nums(raw_wins), parse_nums(raw_picks));
                let count = picks.iter().filter(|n| wins.contains(n)).count() as u32;

                match count {
                    0 => 0,
                    _ => 2_u32.pow(count - 1),
                }
            })
            .sum(),
    )
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut copies: HashMap<usize, u32> = Default::default();

    Some(
        input
            .lines()
            .enumerate()
            .map(|(i, l)| {
                let (raw_wins, raw_picks) =
                    l.split_once(": ").unwrap().1.split_once(" | ").unwrap();
                let (wins, picks) = (parse_nums(raw_wins), parse_nums(raw_picks));
                let won_amount = picks.iter().filter(|n| wins.contains(n)).count();

                let copies_amount = *copies.get(&i).unwrap_or(&0);
                for j in (i + 1)..(i + 1 + won_amount) {
                    *(copies.entry(j).or_default()) += 1 + copies_amount;
                }

                copies_amount + 1
            })
            .sum(),
    )
}

fn parse_nums(raw: &str) -> Vec<u32> {
    raw.split(" ").filter_map(|v| v.parse().ok()).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file_part(
            "examples", DAY, 1,
        ));
        assert_eq!(result, Some(13));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file_part(
            "examples", DAY, 2,
        ));
        assert_eq!(result, Some(30));
    }
}
