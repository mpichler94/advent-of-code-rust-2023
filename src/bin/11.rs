use itertools::Itertools;
use std::collections::HashSet;

advent_of_code::solution!(11);

pub fn part_one(input: &str) -> Option<u32> {
    let image = analyze_image(input, 1);

    let sum: i64 = image.iter().tuple_combinations()
        .map(|(a, b)| a.dist(b))
        .sum();

    Some(sum as u32)
}

pub fn part_two(input: &str) -> Option<u64> {
    let image = analyze_image(input, 999999);

    let sum: i64 = image.iter().tuple_combinations()
        .map(|(a, b)| a.dist(b))
        .sum();

    Some(sum as u64)
}


fn analyze_image(input: &str, expansion: u32) -> Vec<Vec2> {
    let lines = input.trim().lines().collect_vec();
    let mut empty_rows: HashSet<usize> = HashSet::from_iter((0..lines.len()).collect_vec());
    let mut empty_cols: HashSet<usize> = HashSet::from_iter((0..lines.first().unwrap().len()).collect_vec());
    let mut galaxies: Vec<Vec2> = Vec::new();

    for (y, line) in input.trim().lines().enumerate() {
        for (x, c) in line.chars().enumerate() {
            if c == '#' {
                galaxies.push(Vec2 { x: x as i64, y: y as i64 });
                empty_rows.remove(&y);
                empty_cols.remove(&x);
            }
        }
    }

    empty_rows.iter().sorted().rev()
        .for_each(|&y| {
            galaxies.iter_mut()
                .filter(|galaxy| galaxy.y > y as i64)
                .for_each(|galaxy| galaxy.y += expansion as i64)
        });
    empty_cols.iter().sorted().rev()
        .for_each(|&x| {
            galaxies.iter_mut()
                .filter(|galaxy| galaxy.x > x as i64)
                .for_each(|galaxy| galaxy.x += expansion as i64)
        });

    galaxies
}

#[derive(Debug, Hash, Eq, PartialEq, Copy, Clone)]
struct Vec2 {
    x: i64,
    y: i64,
}

impl Vec2 {
    fn dist(&self, other: &Self) -> i64 {
        (self.x - other.x).abs() + (self.y - other.y).abs()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(374));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(82000210));
    }
}
