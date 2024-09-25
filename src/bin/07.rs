use itertools::Itertools;
use std::cmp::Ordering;
use std::cmp::Ordering::{Greater, Less};
use std::collections::HashMap;
use std::iter::Iterator;

advent_of_code::solution!(7);

pub fn part_one(input: &str) -> Option<u32> {
    let hands: Vec<Hand> = input
        .lines()
        .map(|line| {
            let parts: Vec<&str> = line.split_whitespace().collect();
            Hand {
                text: parts[0].to_string(),
                bid: parts[1].parse().unwrap(),
                part_two: false,
            }
        })
        .collect();

    let mut winnings = 0;
    let mut i = 1;
    for hand in hands.iter().sorted() {
        winnings += hand.bid * i;
        i += 1;
    }

    Some(winnings)
}

pub fn part_two(input: &str) -> Option<u32> {
    let hands: Vec<Hand> = input
        .lines()
        .map(|line| {
            let parts: Vec<&str> = line.split_whitespace().collect();
            Hand {
                text: parts[0].to_string(),
                bid: parts[1].parse().unwrap(),
                part_two: true,
            }
        })
        .collect();

    let mut winnings = 0;
    let mut i = 1;
    for hand in hands.iter().sorted() {
        winnings += hand.bid * i;
        i += 1;
    }

    Some(winnings)
}

#[derive(Debug, PartialEq, Eq)]
struct Hand {
    text: String,
    bid: u32,
    part_two: bool,
}

impl Hand {
    fn ordinal(&self) -> u8 {
        if self.part_two {
            self.ordinal_two()
        } else {
            self.ordinal_one()
        }
    }

    fn ordinal_one(&self) -> u8 {
        let char_counts: Vec<usize> = self.text.chars().counts().into_values().collect();
        let of_a_kind = char_counts.iter().max().unwrap();

        if of_a_kind == &5 {
            7
        } else if of_a_kind == &4 {
            6
        } else if char_counts.contains(&3) && char_counts.contains(&2) {
            5
        } else if of_a_kind == &3 {
            4
        } else if char_counts.iter().filter(|it| **it == 2).count() == 2 {
            3
        } else if of_a_kind == &2 {
            2
        } else {
            1
        }
    }

    fn ordinal_two(&self) -> u8 {
        let char_counts: Vec<usize> = self
            .text
            .chars()
            .filter(|it| *it != 'J')
            .counts()
            .into_values()
            .collect();
        let joker_count: usize = self.text.chars().filter(|it| *it == 'J').count();
        let of_a_kind = char_counts.iter().max().unwrap_or(&0) + joker_count;
        let pairs = char_counts.iter().filter(|it| **it == 2).count();

        if of_a_kind == 5 {
            7
        } else if of_a_kind == 4 {
            6
        } else if (char_counts.contains(&3) && char_counts.contains(&2))
            || (pairs == 2 && joker_count == 1)
        {
            5
        } else if of_a_kind == 3 {
            4
        } else if pairs == 2 {
            3
        } else if of_a_kind == 2 {
            2
        } else {
            1
        }
    }
}
impl Ord for Hand {
    fn cmp(&self, other: &Self) -> Ordering {
        if self.text == other.text {
            return Ordering::Equal;
        }

        let card_map: HashMap<char, u8> = HashMap::from([
            ('A', 14),
            ('K', 13),
            ('Q', 12),
            ('J', 11),
            ('T', 10),
            ('9', 9),
            ('8', 8),
            ('7', 7),
            ('6', 6),
            ('5', 5),
            ('4', 4),
            ('3', 3),
            ('2', 2),
            ('1', 1),
        ]);

        let ordinal = self.ordinal();
        let other_ordinal = other.ordinal();

        if ordinal > other_ordinal {
            return Greater;
        }
        if ordinal < other_ordinal {
            return Less;
        }

        let chars: Vec<char>;
        let other_chars: Vec<char>;
        if self.part_two {
            chars = self.text.replace("J", "1").chars().collect();
            other_chars = other.text.replace("J", "1").chars().collect();
        } else {
            chars = self.text.chars().collect();
            other_chars = other.text.chars().collect();
        }
        for i in 0..5 {
            if chars[i] != other_chars[i] {
                return card_map[&chars[i]].cmp(&card_map[&other_chars[i]]);
            }
        }

        Ordering::Equal
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(6440));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(5905));
    }
}
