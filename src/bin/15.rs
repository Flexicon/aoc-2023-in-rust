advent_of_code::solution!(15);

pub fn part_one(input: &str) -> Option<u32> {
    let sum = input
        .split(',')
        .map(|s| {
            s.chars()
                .filter(|c| c != &'\n')
                .fold(0, |sum, c| ((sum + c as u32) * 17) % 256)
        })
        .sum();

    Some(sum)
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
        assert_eq!(result, Some(1320));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
