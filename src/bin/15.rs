use itertools::{enumerate, Itertools};

advent_of_code::solution!(15);

pub fn part_one(input: &str) -> Option<u32> {
    let sum = input.trim().split(',').map(hash_seq).sum();

    Some(sum)
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut boxes: Vec<Vec<(&str, u32)>> = vec![vec![]; 256];

    // Calculate steps
    let steps = input
        .trim()
        .split(',')
        .map(|s| {
            let (seq, len) = s.split_once(|c| matches!(c, '=' | '-')).unwrap();
            (hash_seq(seq) as usize, seq, len.parse::<u32>().ok())
        })
        .collect::<Vec<_>>();

    // Perform steps, filling boxes as appropriate
    for (box_num, label, focal) in steps {
        let b = boxes.get_mut(box_num).unwrap();
        let found = b.iter().find_position(|(s, _)| s == &label);

        match found {
            Some((i, _)) => {
                b.remove(i);
                if let Some(focal) = focal {
                    b.insert(i, (label, focal));
                }
            }
            None => {
                if let Some(focal) = focal {
                    b.push((label, focal))
                }
            }
        }
    }

    let mut sum = 0;
    for (i, b) in enumerate(boxes) {
        for (j, (_, focal)) in enumerate(b) {
            sum += (i as u32 + 1) * (j as u32 + 1) * focal;
        }
    }
    Some(sum)
}

fn hash_seq(seq: &str) -> u32 {
    seq.chars().fold(0, |sum, c| ((sum + c as u32) * 17) % 256)
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
        assert_eq!(result, Some(145));
    }
}
