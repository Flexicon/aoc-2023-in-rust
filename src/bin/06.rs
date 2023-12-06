advent_of_code::solution!(6);

pub fn part_one(input: &str) -> Option<u32> {
    let (line1, line2) = input.split_once("\n").unwrap();

    let (times, records) = (
        line1.split(" ").filter_map(|v| v.parse::<u32>().ok()),
        line2.split(" ").filter_map(|v| v.parse::<u32>().ok()),
    );

    Some(
        times
            .zip(records)
            .map(|(time, record)| (1..time - 1).filter(|i| i * (time - i) > record).count())
            .fold(1, |sum, n| sum * n as u32),
    )
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
        assert_eq!(result, Some(288));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
