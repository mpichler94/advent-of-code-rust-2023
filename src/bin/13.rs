use itertools::Itertools;

advent_of_code::solution!(13);

pub fn part_one(input: &str) -> Option<u32> {
    let patterns = input.split("\n\n");

    let mut sum = 0;
    for pattern in patterns {
        if let Some(res) = find_horizontal_line(pattern, 0) {
            sum += res * 100;
        }
        if let Some(res) = find_vertical_line(pattern, 0) {
            sum += res;
        }
    }

    Some(sum as u32)
}

pub fn part_two(input: &str) -> Option<u32> {
    let patterns = input.split("\n\n");

    let mut sum = 0;
    for pattern in patterns {
        if let Some(res) = find_horizontal_line(pattern, 1) {
            sum += res * 100;
        }
        if let Some(res) = find_vertical_line(pattern, 1) {
            sum += res;
        }
    }

    Some(sum as u32)
}

fn find_horizontal_line(pattern: &str, allowed_defects: u8) -> Option<usize> {
    let lines = pattern.lines().collect_vec();
    let len = lines.first().unwrap().len();
    for i in 0..lines.len() - 1 {
        let mut defects = 0;
        for j in 0..=i {
            if i + 1 + j >= lines.len() {
                if defects == allowed_defects {
                    return Some(i + 1);
                } else {
                    break;
                };
            }

            let mut l = lines[i - j].chars();
            let mut r = lines[i + 1 + j].chars();
            for _ in 0..len {
                if l.next().unwrap() != r.next().unwrap() {
                    defects += 1;
                }
            }
        }

        if defects == allowed_defects {
            return Some(i + 1);
        }
    }

    None
}

fn transpose_pattern(pattern: &str) -> String {
    let columns = pattern.lines().collect_vec();
    let len = columns.first().unwrap().len();
    let mut lines = Vec::<String>::new();

    for y in 0..len {
        let mut s = String::with_capacity(columns.len());
        for column in &columns {
            s.push(column.chars().nth(y).unwrap());
        }
        lines.push(s);
    }

    lines.join("\n")
}

fn find_vertical_line(pattern: &str, allowed_defects: u8) -> Option<usize> {
    let transposed = transpose_pattern(pattern);
    find_horizontal_line(transposed.as_str(), allowed_defects)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(405));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(400));
    }
}
