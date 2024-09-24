extern crate core;

use core::ops::Range;
use std::cmp::{max, min};
use regex::Regex;

advent_of_code::solution!(5);

pub fn part_one(input: &str) -> Option<u32> {
    let re = Regex::new(r"seeds: ((?:\d+| )+)").unwrap();
    let captures = re.captures(input).unwrap();
    let seeds: Vec<u64> = captures[1].split_whitespace().map(|num| num.parse().unwrap()).collect();

    let maps = from_input(input);

    let min = map(&maps, seeds);

    Some(min as u32)
}

pub fn part_two(input: &str) -> Option<u32> {
    let re = Regex::new(r"seeds: ((?:\d+| )+)").unwrap();
    let captures = re.captures(input).unwrap();
    let seed_input: Vec<u64> = captures[1].split_whitespace().map(|num| num.parse().unwrap()).collect();
    
    let mut seeds:  Vec<Range<u64>> = Vec::new();
    for i in 0..seed_input.len() / 2 {
        seeds.push(seed_input[2*i]..seed_input[2*i]+seed_input[2*i+1]);
    }
    
    let maps = from_input(input);

    let min = map_range(&maps, seeds);

    Some(min as u32)
}

fn from_input(input: &str) -> Vec<Map> {
    let seed_to_soil = Map{ mappings: get_mapping(input, "seed-to-soil map") };
    let soil_to_fertilizer = Map{ mappings: get_mapping(input, "soil-to-fertilizer map") };
    let fertilizer_to_water = Map{ mappings: get_mapping(input, "fertilizer-to-water map") };
    let water_to_light = Map{ mappings: get_mapping(input, "water-to-light map") };
    let light_to_temperature = Map{ mappings: get_mapping(input, "light-to-temperature map") } ;
    let temperature_to_humidity = Map{ mappings: get_mapping(input, "temperature-to-humidity map") };
    let humidity_to_location = Map{ mappings: get_mapping(input, "humidity-to-location map") };

    vec![seed_to_soil, soil_to_fertilizer, fertilizer_to_water, water_to_light, light_to_temperature, temperature_to_humidity, humidity_to_location]
}

fn map(maps: &Vec<Map>, seeds: Vec<u64>) -> u64 {
    let mut src = seeds;
    for map in maps {
        src = src.iter().map(|num| map.map(*num)).collect();
    }

    *src.iter().min().unwrap()
}

fn map_range(maps: &Vec<Map>, seeds: Vec<Range<u64>>) -> u64 {
    let mut src = seeds;
    for map in maps {
        src = src.iter().flat_map(|num| map.map_range(num)).collect();
    }

    src.iter().map(|range| range.start).min().unwrap()
}

fn get_mapping(input: &str, name: &str) -> Vec<Mapping> {
    let mut mappings: Vec<Mapping> = Vec::new();
    let pattern = format!("{}:\\n((?:(?: |\\d+)+\\n)+)", name);
    let re = Regex::new(&pattern).unwrap();
    let captures = re.captures(input).unwrap();
    let lines = captures[1].lines();
    for line in lines {
        let numbers: Vec<&str> = line.split_whitespace().collect();
        mappings.push(Mapping{src: numbers[1].parse().unwrap(), dest: numbers[0].parse().unwrap(), len: numbers[2].parse().unwrap()})
    }

    mappings
}

#[derive(Debug, Hash, Eq, PartialEq)]
struct Map{mappings: Vec<Mapping>}

impl Map{
    fn map(&self, num: u64) -> u64 {
        self.mappings.iter()
            .filter_map(|mapping| mapping.map(num))
            .next()
            .unwrap_or(num)
    }

    fn map_range(&self, num: &Range<u64>) -> Vec<Range<u64>> {
        let mut res: Vec<Range<u64>> = Vec::new();

        for mapping in self.mappings.iter() {
            if let Some(dest) = mapping.map_range(num) {
                res.push(dest);
            }
        }

        let mut s = num.start;
        let mut e = num.end;
        loop {
            let mut changed = false;
            for mapping in self.mappings.iter() {
                if mapping.src < s && mapping.src < e && mapping.src + mapping.len > s && mapping.src + mapping.len < e {
                    s = mapping.src + mapping.len;
                    changed = true;
                }
                if mapping.src > s && mapping.src < e {
                    e = mapping.src;
                    changed = true;
                }
            }
            
            if changed {
                res.push(s..e);
            } else {
                break
            }
        }
        
        if res.is_empty() {
            res.push(num.start..num.end);
        }

        res
    }
}

#[derive(Debug, Hash, Eq, PartialEq)]
struct Mapping{src: u64, dest: u64, len: u64}

impl Mapping{
    fn map(&self, num: u64) -> Option<u64> {
        if num < self.src || num > self.src + self.len {
            None
        } else {
            Some(self.dest + num - self.src)
        }
    }

    fn map_range(&self, num: &Range<u64>) -> Option<Range<u64>> {
        if num.end < self.src || num.start > self.src + self.len {
            None
        } else {
            let start = max(self.src, num.start);
            let end = min(self.src + self.len, num.end);
            Some(self.dest + start - self.src..(self.dest + start - self.src) + end - start)
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(35));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(46));
    }
}
