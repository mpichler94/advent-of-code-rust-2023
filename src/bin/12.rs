use itertools::Itertools;
use memoize::memoize;

advent_of_code::solution!(12);

pub fn part_one(input: &str) -> Option<u32> {
    let records = input.trim().lines().map(|line| {
        let (pattern, numbers) = line.split_whitespace().collect_tuple().unwrap();
        (pattern, numbers.split(",").map(|num| num.parse::<usize>().unwrap()).collect_vec())
    }).collect_vec();

    let mut arrangements = 0;
    for record in records {
        arrangements += replace(record.0.to_string(), record.1, '0');
    }

    Some(arrangements as u32)
}

pub fn part_two(input: &str) -> Option<u64> {
    let records = input.trim().lines().map(|line| {
        let (pattern, numbers) = line.split_whitespace().collect_tuple().unwrap();
        let repeated_pattern = (0..5).map(|_| pattern).join("?");
        let repeated_numbers = numbers.split(",").map(|num| num.parse::<usize>().unwrap()).collect_vec().repeat(5);
        (repeated_pattern, repeated_numbers)
    }).collect_vec();

    let mut arrangements = 0;
    for record in records {
        arrangements += replace(record.0, record.1, '0');
    }

    Some(arrangements)
}

#[memoize]
fn replace(row: String, numbers: Vec<usize>, last_char: char) -> u64 {
    if row.is_empty() {
        return if numbers.is_empty() || (numbers.len() == 1 && numbers.first().unwrap() == &0) {
            1
        } else {
            0
        };
    }

    if let Some(remaining_row) = row.strip_prefix('?') {
        let mut arrangements = 0;
        if numbers.first().unwrap_or(&0) > &0 {
            let mut new_numbers = numbers.clone();
            new_numbers[0] -= 1;
            arrangements += replace(remaining_row.to_string(), new_numbers, '#');
        }
        if numbers.first().unwrap_or(&0) > &0 {
            if last_char == '#' {
                return arrangements;
            }
            arrangements += replace(remaining_row.to_string(), numbers, '.');
        } else {
            let new_numbers = numbers.clone().into_iter().skip(1).collect::<Vec<_>>();
            arrangements += replace(remaining_row.to_string(), new_numbers, '.');
        }
        return arrangements;
    }

    if let Some(remaining_row) = row.strip_prefix('#') {
        let mut new_numbers = numbers.clone();
        if numbers.is_empty() || numbers[0] == 0 {
            return 0;
        }
        new_numbers[0] -= 1;
        return replace(remaining_row.to_string(), new_numbers, '#');
    }

    if let Some(remaining_row) = row.strip_prefix('.') {
        return if numbers.first().unwrap_or(&0) > &0 {
            if last_char == '#' {
                return 0;
            }
            replace(remaining_row.to_string(), numbers, '.')
        } else {
            let new_numbers = numbers.clone().into_iter().skip(1).collect::<Vec<_>>();
            replace(remaining_row.to_string(), new_numbers, '.')
        };
    }

    panic!("Should not reach")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(21));
    }

    #[test]
    fn test_part_one_2() {
        let result = part_one(&advent_of_code::template::read_file_part("examples", DAY, 1));
        assert_eq!(result, Some(7541));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(525152));
    }
}
