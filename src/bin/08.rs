use std::collections::HashMap;

advent_of_code::solution!(8);

pub fn part_one(input: &str) -> Option<u32> {
    let instructions: Vec<char> = input.trim().lines().next().unwrap().chars().collect();
    let map = parse_map(input);

    let steps = find_path(&instructions, &map, "AAA");

    Some(steps as u32)
}

pub fn part_two(input: &str) -> Option<usize> {
    let instructions: Vec<char> = input.trim().lines().next().unwrap().chars().collect();
    let map = parse_map(input);

    let start_nodes: Vec<&&str> = map.keys().filter(|n| n.ends_with("A")).collect();

    let steps: usize = start_nodes
        .iter()
        .map(|n| find_path(&instructions, &map, n))
        .reduce(lcm)
        .unwrap();

    Some(steps)
}

fn find_path(instructions: &[char], map: &HashMap<&str, (&str, &str)>, start: &str) -> usize {
    let mut node = start;
    let mut step: usize = 0;
    while !node.ends_with("Z") {
        let i = step % instructions.len();
        if instructions[i] == 'R' {
            node = map[node].1;
        } else {
            node = map[node].0;
        }
        step += 1;
    }

    step
}

fn parse_map(input: &str) -> HashMap<&str, (&str, &str)> {
    let lines: Vec<&str> = input.trim().lines().collect();
    let mut map = HashMap::new();
    for line in lines.iter().skip(2) {
        let node = parse_line(line);
        map.insert(node.0, node.1);
    }
    map
}

fn parse_line(line: &str) -> (&str, (&str, &str)) {
    let node = line.get(0..3).unwrap();
    let left = line.get(7..10).unwrap();
    let right = line.get(12..15).unwrap();

    (node, (left, right))
}

fn lcm(a: usize, b: usize) -> usize {
    (a / gcd(a, b)) * b
}

fn gcd(a: usize, b: usize) -> usize {
    if b == 0 {
        a
    } else {
        gcd(b, a % b)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(6));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file_part(
            "examples", DAY, 2,
        ));
        assert_eq!(result, Some(6));
    }
}
