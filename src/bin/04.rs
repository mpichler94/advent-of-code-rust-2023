use std::collections::HashSet;
use std::ops::Shl;
use regex::Regex;

advent_of_code::solution!(4);

pub fn part_one(input: &str) -> Option<u32> {
    let cards = create_cards(input);
    
    let points = cards.iter().map(|card| card.points()).sum();
    
    Some(points)
}

pub fn part_two(input: &str) -> Option<u32> {
    let cards = create_cards(input);
    
    let mut copies: Vec<u32> = vec![1; cards.len()];
    
    for i in 0..cards.len() {
        for j in (i + 1)..(i + cards[i].matches() + 1) {
            copies[j] += copies[i];
        }
    }
    
    Some(copies.iter().sum())
}

fn create_cards(input: &str) -> Vec<Card> {
    let mut cards: Vec<Card> = Vec::new();
    let re = Regex::new(r"Card +\d+: +((?:\d+ *)+) \| +((?:\d+ *)+)").unwrap();
    for line in input.trim().lines() {
        let captures = re.captures(line).unwrap();
        let winning_numbers = captures[1].split_whitespace().map(|num| num.parse::<u32>().unwrap()).collect();
        let numbers =  captures[2].split_whitespace().map(|num| num.parse::<u32>().unwrap()).collect();
        cards.push(Card{winning_numbers, numbers})
    }
    
    cards
}

#[derive(Debug, Hash, Eq, PartialEq)]
struct Card{winning_numbers: Vec<u32>, numbers: Vec<u32>}

impl Card {
    fn matches(&self) -> usize {
        let a: HashSet<_> = HashSet::from_iter(self.winning_numbers.iter());
        let b = HashSet::from_iter(self.numbers.iter());
        let intersection: HashSet<_> = a.intersection(&b).collect();
        
        intersection.len()
    }
    
    fn points(&self) -> u32 {
        let matches = self.matches();
        if matches == 0 {
            0
        } else {
            1u32.shl(matches - 1)
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(13));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(30));
    }
}
