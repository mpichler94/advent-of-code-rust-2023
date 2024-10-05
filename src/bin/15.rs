advent_of_code::solution!(15);

pub fn part_one(input: &str) -> Option<u32> {
    let instructions = input.trim().split(',');

    let sum = instructions.map(|instruction| hash(instruction) as u32).sum();

    Some(sum)
}

pub fn part_two(input: &str) -> Option<usize> {
    let instructions = input.trim().split(',');

    let mut boxes = Vec::new();
    for _ in 0..256 {
        boxes.push(Vec::<Lens>::new())
    }
    for instruction in instructions {
        do_instruction(instruction, &mut boxes)
    }

    let mut focusing_power = 0;
    for (i, lenses) in boxes.into_iter().enumerate() {
        for (j, len) in lenses.into_iter().enumerate() {
            focusing_power += (i + 1) * (j + 1) * len.focal_length as usize;
        }
    }

    Some(focusing_power)
}

fn hash(input: &str) -> u8 {
    let mut value = 0;

    for c in input.chars() {
        value += c as u16;
        value *= 17;
        value %= 256;
    }

    value as u8
}

fn do_instruction(instruction: &str, boxes: &mut [Box]) {
    let mut cmd = instruction.to_string();
    let suffix = cmd.pop().unwrap();
    if suffix != '-' {
        cmd.pop();
    }
    let label = cmd.clone();
    let id = hash(&label);
    let mut pos = None;

    for (i, lens) in boxes[id as usize].iter().enumerate() {
        if lens.label == label {
            pos = Some(i);
            break;
        }
    }
    if let Some(i) = pos {
        boxes[id as usize].remove(i);
    }

    if suffix != '-' {
        let focal_length = suffix.to_digit(10).unwrap();

        let lens = Lens { label, focal_length: focal_length as u8 };
        if let Some(i) = pos {
            boxes[id as usize].insert(i, lens);
        } else {
            boxes[id as usize].push(lens);
        }
    }
}

type Box = Vec<Lens>;

struct Lens {
    label: String,
    focal_length: u8,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(1320));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(145));
    }
}
