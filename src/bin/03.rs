advent_of_code::solution!(3);

pub fn part_one(input: &str) -> Option<u32> {
    let (symbols, numbers) = parse_input(input);

    let result: i32 = numbers
        .iter()
        .filter(|num| symbols.iter().any(|sym| num.is_adjacent(sym)))
        .map(|num| num.value)
        .sum();

    Some(result as u32)
}

pub fn part_two(input: &str) -> Option<u32> {
    let (symbols, numbers) = parse_input(input);

    let result: i32 = symbols
        .iter()
        .filter(|symbol| symbol.value == '*')
        .map(|symbol| {
            let products: Vec<i32> = numbers
                .iter()
                .filter(|num| num.is_adjacent(symbol))
                .map(|num| num.value)
                .collect();

            if products.len() == 2 {
                products.iter().fold(1, |acc, prod| acc * prod)
            } else {
                0i32
            }
        })
        .sum();

    Some(result as u32)
}

fn parse_input(input: &str) -> (Vec<Symbol>, Vec<Number>) {
    let mut numbers = Vec::new();
    let mut symbols = Vec::new();
    let mut lines = input.trim().lines();
    let mut y = 0;
    while let Some(line) = lines.next() {
        let mut value = String::new();
        let mut chars = line.chars();
        let mut x = 0;
        while let Some(c) = chars.next() {
            if !c.is_digit(10) && !value.is_empty() {
                numbers.push(Number {
                    value: value.parse().unwrap(),
                    x: x - value.len() as i32,
                    y,
                });
                value.clear();
            } else if c.is_digit(10) {
                value.push_str(&c.to_string());
            }
            if c != '.' && !c.is_digit(10) {
                symbols.push(Symbol { value: c, x, y })
            }
            x += 1;
        }
        if !value.is_empty() {
            numbers.push(Number {
                value: value.parse().unwrap(),
                x: line.len() as i32 - value.len() as i32,
                y,
            })
        }
        y += 1;
    }

    (symbols, numbers)
}

#[derive(Debug)]
struct Symbol {
    value: char,
    x: i32,
    y: i32,
}

#[derive(Debug)]
struct Number {
    value: i32,
    x: i32,
    y: i32,
}

impl Number {
    fn is_adjacent(&self, pos: &Symbol) -> bool {
        if pos.y < self.y - 1 || pos.y > self.y + 1 {
            false
        } else if pos.x < self.x - 1 || pos.x > self.x + self.value.to_string().len() as i32 {
            false
        } else {
            true
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(4361));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(467835));
    }
}
