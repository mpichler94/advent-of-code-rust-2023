use itertools::Itertools;
use std::collections::HashSet;

advent_of_code::solution!(16);

pub fn part_one(input: &str) -> Option<u32> {
    let lines = input.trim().lines().collect_vec();

    let energized = fire_beam(Beam { pos: (0, 0), dir: Direction::Right }, &lines);

    Some(energized as u32)
}

pub fn part_two(input: &str) -> Option<u32> {
    let lines = input.trim().lines().collect_vec();
    let height = lines.len();
    let width = lines[0].len();

    let mut energized = Vec::<usize>::new();
    for x in 0..width {
        energized.push(fire_beam(Beam { pos: (x, 0), dir: Direction::Down }, &lines));
        energized.push(fire_beam(Beam { pos: (x, height - 1), dir: Direction::Up }, &lines));
    }
    for y in 0..height {
        energized.push(fire_beam(Beam { pos: (0, y), dir: Direction::Right }, &lines));
        energized.push(fire_beam(Beam { pos: (width - 1, y), dir: Direction::Left }, &lines));
    }

    Some(*energized.iter().max().unwrap() as u32)
}

fn fire_beam(beam: Beam, lines: &[&str]) -> usize {
    let height = lines.len();
    let width = lines[0].len();

    let mut processed_beams = HashSet::<Beam>::new();
    let mut beams = Vec::<Beam>::new();
    let mut energized = HashSet::<Pos>::new();
    beams.push(beam);

    while let Some(mut beam) = beams.pop() {
        let pos = beam.pos;

        energized.insert(pos);
        processed_beams.insert(Beam { pos, dir: beam.dir });
        if let Some(additional_beam) = process_tile(&mut beam, lines) {
            beams.push(additional_beam);
        }

        while let Some(next) = beam.next(width, height) {
            beam = Beam { pos: next, dir: beam.dir };
            if processed_beams.contains(&beam) {
                break;
            }
            energized.insert(next);
            processed_beams.insert(Beam { pos: next, dir: beam.dir });
            if let Some(additional_beam) = process_tile(&mut beam, lines) {
                beams.push(additional_beam);
            }
        }
    }

    energized.len()
}

fn process_tile(beam: &mut Beam, lines: &[&str]) -> Option<Beam> {
    let c = lines[beam.pos.1].chars().nth(beam.pos.0).unwrap();
    if c == '|' && (beam.dir == Direction::Left || beam.dir == Direction::Right) {
        beam.dir = Direction::Up;
        Some(Beam { pos: beam.pos, dir: Direction::Down })
    } else if c == '-' && (beam.dir == Direction::Down || beam.dir == Direction::Up) {
        beam.dir = Direction::Left;
        Some(Beam { pos: beam.pos, dir: Direction::Right })
    } else if c == '\\' {
        match beam.dir {
            Direction::Up => beam.dir = Direction::Left,
            Direction::Down => beam.dir = Direction::Right,
            Direction::Left => beam.dir = Direction::Up,
            Direction::Right => beam.dir = Direction::Down,
        }
        None
    } else if c == '/' {
        match beam.dir {
            Direction::Up => beam.dir = Direction::Right,
            Direction::Down => beam.dir = Direction::Left,
            Direction::Left => beam.dir = Direction::Down,
            Direction::Right => beam.dir = Direction::Up,
        }
        None
    } else {
        None
    }
}


type Pos = (usize, usize);

#[derive(Debug, Hash, Eq, PartialEq, Copy, Clone)]
enum Direction { Up, Down, Left, Right }

#[derive(Debug, Hash, Eq, PartialEq)]
struct Beam {
    pos: Pos,
    dir: Direction,
}

impl Beam {
    fn next(&self, width: usize, height: usize) -> Option<Pos> {
        match self.dir {
            Direction::Up => {
                if self.pos.1 > 0 {
                    Some((self.pos.0, self.pos.1 - 1))
                } else {
                    None
                }
            }
            Direction::Down => {
                if self.pos.1 < height - 1 {
                    Some((self.pos.0, self.pos.1 + 1))
                } else {
                    None
                }
            }
            Direction::Left => {
                if self.pos.0 > 0 {
                    Some((self.pos.0 - 1, self.pos.1))
                } else {
                    None
                }
            }
            Direction::Right => {
                if self.pos.0 < width - 1 {
                    Some((self.pos.0 + 1, self.pos.1))
                } else {
                    None
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(46));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(51));
    }
}
