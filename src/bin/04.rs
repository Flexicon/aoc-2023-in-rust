advent_of_code::solution!(4);

pub fn part_one(input: &str) -> Option<u32> {
    Some(
        input
            .lines()
            .map(|l| {
                let (raw_wins, raw_picks) =
                    l.split_once(": ").unwrap().1.split_once(" | ").unwrap();
                let (wins, picks) = (parse_nums(raw_wins), parse_nums(raw_picks));
                let won_amount = picks.iter().filter(|n| wins.contains(n)).count();

                match won_amount {
                    0 => 0,
                    _ => 2_u32.pow((won_amount - 1).try_into().unwrap()),
                }
            })
            .sum(),
    )
}

pub fn part_two(_: &str) -> Option<u32> {
    None
}

fn parse_nums(raw: &str) -> Vec<u32> {
    raw.split(" ").filter_map(|v| v.parse().ok()).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(13));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
