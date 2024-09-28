use itertools::enumerate;
use std::cmp::min;
use std::collections::{HashMap, HashSet};
use std::hash::Hash;

advent_of_code::solution!(10);

pub fn part_one(input: &str) -> Option<u32> {
    let map = build_map(input);

    let mut distances: HashMap<Vec2, usize> = HashMap::new();
    walk(&map, map.pipes[&map.start].1, &mut distances);
    walk(&map, map.pipes[&map.start].0, &mut distances);

    Some(*distances.values().max().unwrap() as u32)
}

pub fn part_two(input: &str) -> Option<u32> {
    let field = remove_open_pipes(input);
    let mut inside = false;
    let mut count = 0;
    for line in field.lines() {
        for c in line.chars() {
            if c == '|' || c == 'F' || c == '7' {
                inside = !inside;
            }

            if inside && c == '.' {
                count += 1;
            }
        }
    }

    Some(count)
}

fn walk(map: &Map, first_node: Vec2, distances: &mut HashMap<Vec2, usize>) -> HashSet<Vec2> {
    let mut visited: HashSet<Vec2> = HashSet::new();
    distances.insert(map.start, 0);
    visited.insert(map.start);

    let mut node = first_node;
    let mut dist = 0;
    while node != map.start {
        dist += 1;
        if distances.contains_key(&node) {
            distances.insert(node, min(distances[&node], dist));
        } else {
            distances.insert(node, dist);
        }
        visited.insert(node);
        let next = map.pipes[&node].1;
        if visited.contains(&next) {
            if visited.contains(&map.pipes[&node].0) {
                if map.pipes[&node].0 == map.start || map.pipes[&node].1 == map.start {
                    return visited;
                }
                panic!("LOOP")
            }
            node = map.pipes[&node].0;
        } else {
            node = next;
        }
    }

    visited
}

fn build_map(input: &str) -> Map {
    let mut pipes: HashMap<Vec2, (Vec2, Vec2)> = HashMap::new();
    let mut start: Vec2 = Vec2 { x: 0, y: 0 };
    let lines: Vec<&str> = input.trim().lines().collect();
    for (ye, line) in enumerate(lines) {
        let y = ye as i32;
        for (xe, c) in enumerate(line.chars()) {
            let x = xe as i32;
            match c {
                '|' => pipes.insert(Vec2 { x, y }, (Vec2 { x, y: y - 1 }, Vec2 { x, y: y + 1 })),
                '-' => pipes.insert(Vec2 { x, y }, (Vec2 { x: x - 1, y }, Vec2 { x: x + 1, y })),
                'L' => pipes.insert(Vec2 { x, y }, (Vec2 { x, y: y - 1 }, Vec2 { x: x + 1, y })),
                'J' => pipes.insert(Vec2 { x, y }, (Vec2 { x: x - 1, y }, Vec2 { x, y: y - 1 })),
                '7' => pipes.insert(Vec2 { x, y }, (Vec2 { x: x - 1, y }, Vec2 { x, y: y + 1 })),
                'F' => pipes.insert(Vec2 { x, y }, (Vec2 { x, y: y + 1 }, Vec2 { x: x + 1, y })),
                'S' => {
                    start = Vec2 { x, y };
                    continue;
                }
                _ => continue,
            };
        }
    }

    let neighbors: Vec<Vec2> = start
        .adjacent()
        .into_iter()
        .filter(|n| {
            if !pipes.contains_key(n) {
                return false;
            }
            let neighbors = pipes.get(n).unwrap();
            neighbors.0 == start || neighbors.1 == start
        })
        .collect();
    pipes.insert(start, (neighbors[0], neighbors[1]));

    Map { start, pipes }
}

fn get_start_char(map: &Map) -> char {
    let start = map.pipes.get(&map.start).unwrap();
    if start.0.x == map.start.x && start.1.x == map.start.x {
        return '|';
    } else if start.0.y == map.start.y && start.1.y == map.start.y {
        return '-';
    } else if start == &(Vec2 { x: map.start.x, y: map.start.y - 1 }, Vec2 { x: map.start.x + 1, y: map.start.y }) {
        return 'L';
    } else if start == &(Vec2 { x: map.start.x - 1, y: map.start.y }, Vec2 { x: map.start.x, y: map.start.y - 1 }) {
        return 'J';
    } else if start == &(Vec2 { x: map.start.x - 1, y: map.start.y }, Vec2 { x: map.start.x, y: map.start.y + 1 }) {
        return '7';
    } else if start == &(Vec2 { x: map.start.x + 1, y: map.start.y }, Vec2 { x: map.start.x, y: map.start.y + 1 }) {
        return 'F';
    }

    panic!("CANNOT RESOLVE START!")
}

fn remove_open_pipes(input: &str) -> String {
    let map = build_map(input);
    let mut distances: HashMap<Vec2, usize> = HashMap::new();
    let visited = walk(&map, map.pipes.get(&map.start).unwrap().1, &mut distances);
    let start_char = get_start_char(&map);

    let mut new_input = String::new();
    for (y, line) in enumerate(input.trim().replace("S", &start_char.to_string()).lines()) {
        for (x, c) in enumerate(line.chars()) {
            if visited.contains(&Vec2 { x: x as i32, y: y as i32 }) {
                new_input.push(c);
            } else {
                new_input.push('.');
            }
        }
        new_input.push('\n');
    }

    new_input
}

#[derive(Debug, Hash, Eq, PartialEq, Copy, Clone)]
struct Vec2 {
    x: i32,
    y: i32,
}

impl Vec2 {
    fn adjacent(&self) -> Vec<Vec2> {
        vec![
            Vec2 { x: self.x - 1, y: self.y },
            Vec2 { x: self.x, y: self.y - 1 },
            Vec2 { x: self.x + 1, y: self.y },
            Vec2 { x: self.x, y: self.y + 1 },
        ]
    }
}

#[derive(Debug, Eq, PartialEq)]
struct Map {
    start: Vec2,
    pipes: HashMap<Vec2, (Vec2, Vec2)>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(8));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file_part(
            "examples", DAY, 2,
        ));
        assert_eq!(result, Some(4));
    }

    #[test]
    fn test_part_two_2() {
        let result = part_two(&advent_of_code::template::read_file_part(
            "examples", DAY, 3,
        ));
        assert_eq!(result, Some(10));
    }
}
