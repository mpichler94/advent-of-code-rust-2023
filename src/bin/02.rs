advent_of_code::solution!(2);

use regex::Regex;

pub fn part_one(input: &str) -> Option<u32> {
    let ids = input
        .trim()
        .lines()
        .map(|line| from_input(line))
        .filter_map(|game| {
            if game.valid() {
                Some(u32::from(game.id))
            } else {
                None
            }
        })
        .sum();

    Some(ids)
}

pub fn part_two(input: &str) -> Option<u32> {
    let games = input
        .trim()
        .lines()
        .map(|line| from_input(line))
        .map(|game| game.power())
        .sum();

    Some(games)
}

fn from_input(input: &str) -> Game {
    let re = Regex::new(r"Game (\d+): (.*)").unwrap();

    let caps = re.captures(input).unwrap();
    let id = caps[1].parse().unwrap();
    let r = caps[2].split("; ");
    let mut rounds: Vec<Round> = Vec::new();
    for round in r {
        let mut red = 0;
        let mut green = 0;
        let mut blue = 0;

        let cubes = round.split(", ");
        for cube in cubes {
            if cube.ends_with("red") {
                red += cube.replace(" red", "").parse::<u8>().unwrap();
            } else if cube.ends_with("green") {
                green += cube.replace(" green", "").parse::<u8>().unwrap();
            } else if cube.ends_with("blue") {
                blue += cube.replace(" blue", "").parse::<u8>().unwrap();
            }
        }
        rounds.push(Round { red, green, blue })
    }

    Game { id, rounds }
}

struct Game {
    id: u8,
    rounds: Vec<Round>,
}

impl Game {
    fn valid(&self) -> bool {
        self.rounds.iter().all(|r| r.valid())
    }

    fn min_red(&self) -> u8 {
        self.rounds.iter().max_by_key(|r| r.red).unwrap().red
    }
    fn min_green(&self) -> u8 {
        self.rounds.iter().max_by_key(|r| r.green).unwrap().green
    }
    fn min_blue(&self) -> u8 {
        self.rounds.iter().max_by_key(|r| r.blue).unwrap().blue
    }

    fn power(&self) -> u32 {
        u32::from(self.min_red()) * u32::from(self.min_green()) * u32::from(self.min_blue())
    }
}

struct Round {
    red: u8,
    green: u8,
    blue: u8,
}

impl Round {
    fn sum(&self) -> u32 {
        u32::from(self.red) + u32::from(self.green) + u32::from(self.blue)
    }

    fn valid(&self) -> bool {
        !(self.sum() > 39 || self.red > 12 || self.green > 13 || self.blue > 14)
    }
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
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2286));
    }
}
