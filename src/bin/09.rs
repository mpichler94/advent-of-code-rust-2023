advent_of_code::solution!(9);

pub fn part_one(input: &str) -> Option<i64> {
    let readings = get_readings(input);

    let sum: i64 = readings
        .iter()
        .map(|reading| extrapolate_back(reading))
        .sum();

    Some(sum)
}

pub fn part_two(input: &str) -> Option<i64> {
    let readings = get_readings(input);

    let sum: i64 = readings
        .iter()
        .map(|reading| extrapolate_front(reading))
        .sum();

    Some(sum)
}

fn get_readings(input: &str) -> Vec<Vec<i64>> {
    let readings: Vec<Vec<i64>> = input
        .lines()
        .map(|line| {
            line.split_whitespace()
                .map(|num| num.parse::<i64>().unwrap())
                .collect()
        })
        .collect();
    readings
}

fn extrapolate_back(history: &[i64]) -> i64 {
    let mut last_values: Vec<i64> = Vec::new();
    last_values.push(*history.last().unwrap());

    let mut diffs: Vec<i64> = history.windows(2).map(|w| w[1] - w[0]).collect();
    let mut diff_sum: i64 = diffs.iter().sum();

    while diff_sum != 0 {
        last_values.push(*diffs.last().unwrap());
        diffs = diffs.windows(2).map(|w| w[1] - w[0]).collect();
        diff_sum = diffs.iter().sum();
    }

    last_values.iter().sum()
}

fn extrapolate_front(history: &[i64]) -> i64 {
    let mut first_values: Vec<i64> = Vec::new();
    first_values.push(*history.first().unwrap());

    let mut diffs: Vec<i64> = history.windows(2).map(|w| w[1] - w[0]).collect();
    let mut diff_sum: i64 = diffs.iter().sum();

    while diff_sum != 0 {
        first_values.push(*diffs.first().unwrap());
        diffs = diffs.windows(2).map(|w| w[1] - w[0]).collect();
        diff_sum = diffs.iter().sum();
    }

    first_values.into_iter().rev().fold(0, |acc, v| v - acc)
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
        assert_eq!(result, Some(2));
    }
}
