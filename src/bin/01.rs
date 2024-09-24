use std::collections::HashMap;

advent_of_code::solution!(1);

pub fn part_one(input: &str) -> Option<u32> {
    let lines = input.trim().lines();

    let result: u32 = lines.map(|it| {
            let txt = it.chars().filter(|c| c.is_numeric()).collect::<Vec<char>>();
            let num = format!("{}{}", txt[0], txt[txt.len() - 1]);
            num.parse::<u32>().unwrap()
        }).sum();

    Some(result)
}

pub fn part_two(input: &str) -> Option<u32> {
    let lines = input.trim().lines();

    let result = lines.map(find_number).sum();

    Some(result)
}

fn find_number(text: &str) -> u32 {
    let digits: HashMap<&str,u32> = HashMap::from([
        ("one", 1),
        ("two", 2),
        ("three", 3),
        ("four", 4),
        ("five", 5),
        ("six", 6),
        ("seven", 7),
        ("eight", 8),
        ("nine", 9)
    ]);

    let mut first_digit: Option<u32> = None;
    let mut second_digit: Option<u32> = None;
    for i in 0..text.len() {
        digits.iter().for_each(|(k, v)| {
            if first_digit.is_none() {
                if text.chars().nth(i).unwrap().is_ascii_digit() {
                    first_digit = text.chars().nth(i).unwrap().to_digit(10);
                } else if text.get(i..).unwrap().starts_with(k) {
                    first_digit = Some(*v);
                }
            }
            if second_digit.is_none() {
                if text.chars().nth(text.len() - i - 1).unwrap().is_ascii_digit() {
                    second_digit = text.chars().nth_back(i).unwrap().to_digit(10);
                } else if text.get((text.len() - i - 1)..).unwrap().starts_with(k) {
                    second_digit = Some(*v);
                }
            }
        });
    }
    first_digit.unwrap() * 10 + second_digit.unwrap()
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result.unwrap(), 142);
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file_part("examples", DAY, 2));
        assert_eq!(result.unwrap(), 281);
    }
}
