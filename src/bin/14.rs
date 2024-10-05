use itertools::{enumerate, Itertools};
use std::cmp::Ordering;
use std::collections::HashMap;

advent_of_code::solution!(14);

pub fn part_one(input: &str) -> Option<u32> {
    let field = parse_input(input);

    let rocks = tilt(&field.rocks, Direction::Up, field.width, field.height);

    let load = compute_load(rocks, field.height);

    Some(load as u32)
}

pub fn part_two(input: &str) -> Option<u32> {
    let field = parse_input(input);

    let cycle = find_cycle(&field);

    let remainder = (1_000_000_000 - (cycle.start + cycle.length)) % (cycle.length);
    let mut rocks = cycle.rocks;
    let mut step = 0;
    while step < remainder {
        rocks = do_cycle(&rocks, field.width, field.height);
        step += 1;
    }

    let load = compute_load(rocks, field.height);

    Some(load as u32)
}

fn parse_input(input: &str) -> Field {
    let mut rocks = Vec::<Rock>::new();
    let lines = input.trim().lines().collect_vec();
    let height = lines.len();
    let width = lines.first().unwrap().len();

    for (y, line) in enumerate(lines) {
        for (x, c) in line.chars().enumerate() {
            match c {
                'O' => rocks.push(Rock { x, y, round: true }),
                '#' => rocks.push(Rock { x, y, round: false }),
                _ => {}
            }
        }
    }

    Field { width, height, rocks }
}

#[derive(Debug, Hash, Eq, PartialEq)]
struct Field {
    width: usize,
    height: usize,
    rocks: Vec<Rock>,
}


fn tilt(rocks: &[Rock], direction: Direction, width: usize, height: usize) -> Vec<Rock> {
    let mut res = HashMap::<(usize, usize), Rock>::with_capacity(rocks.len());

    let mut rocks = rocks.to_vec();
    match direction {
        Direction::Up => rocks.sort_by(|a, b| a.y.cmp(&b.y)),
        Direction::Down => rocks.sort_by(|a, b| b.y.cmp(&a.y)),
        Direction::Left => rocks.sort_by(|a, b| a.x.cmp(&b.x)),
        Direction::Right => rocks.sort_by(|a, b| b.x.cmp(&a.x)),
    }

    for rock in rocks {
        if !rock.round {
            res.insert((rock.x, rock.y), rock);
            continue;
        }

        if direction == Direction::Up {
            let mut y = rock.y;
            while y > 0 {
                if res.contains_key(&(rock.x, y - 1)) {
                    break;
                }
                y -= 1;
            }
            res.insert((rock.x, y), Rock { x: rock.x, y, round: true });
        } else if direction == Direction::Down {
            let mut y = rock.y;
            while y < height - 1 {
                if res.contains_key(&(rock.x, y + 1)) {
                    break;
                }
                y += 1;
            }
            res.insert((rock.x, y), Rock { x: rock.x, y, round: true });
        } else if direction == Direction::Left {
            let mut x = rock.x;
            while x > 0 {
                if res.contains_key(&(x - 1, rock.y)) {
                    break;
                }
                x -= 1;
            }
            res.insert((x, rock.y), Rock { x, y: rock.y, round: true });
        } else if direction == Direction::Right {
            let mut x = rock.x;
            while x < width - 1 {
                if res.contains_key(&(x + 1, rock.y)) {
                    break;
                }
                x += 1;
            }
            res.insert((x, rock.y), Rock { x, y: rock.y, round: true });
        }
    }

    res.into_values().collect_vec()
}

fn find_cycle(field: &Field) -> Cycle {
    let mut patterns = HashMap::<Vec<Rock>, usize>::new();
    patterns.insert(field.rocks.clone(), 0);

    let mut rocks = do_cycle(&field.rocks, field.width, field.height);
    let mut step = 1;

    while !patterns.contains_key(&rocks) {
        patterns.insert(rocks.clone(), step);
        rocks = do_cycle(&rocks, field.width, field.height);
        step += 1;
    }

    let start = patterns.get(&rocks).unwrap();

    Cycle { start: *start, length: step - start, rocks }
}

struct Cycle {
    start: usize,
    length: usize,
    rocks: Vec<Rock>,
}

fn do_cycle(rocks: &[Rock], width: usize, height: usize) -> Vec<Rock> {
    let mut rocks = rocks.to_vec();
    rocks = tilt(&rocks, Direction::Up, width, height);
    rocks = tilt(&rocks, Direction::Left, width, height);
    rocks = tilt(&rocks, Direction::Down, width, height);
    rocks = tilt(&rocks, Direction::Right, width, height);

    rocks.sort();
    rocks
}

fn compute_load(rocks: Vec<Rock>, height: usize) -> usize {
    let mut load = 0;
    for rock in rocks {
        if !rock.round {
            continue;
        }

        load += height - rock.y;
    }

    load
}

#[derive(Debug, Hash, Eq, PartialEq, Copy, Clone)]
struct Rock {
    x: usize,
    y: usize,
    round: bool,
}

impl Ord for Rock {
    fn cmp(&self, other: &Self) -> Ordering {
        let ord = self.y.cmp(&other.y);
        if ord == Ordering::Equal {
            return self.x.cmp(&other.x);
        }
        ord
    }
}

impl PartialOrd for Rock {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

#[derive(Debug, Hash, Eq, PartialEq, Copy, Clone)]
enum Direction { Up, Down, Left, Right }

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(136));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(64));
    }
}
