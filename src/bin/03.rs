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
                products.iter().product::<i32>()
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
    let lines = input.trim().lines();
    for (y, line) in lines.enumerate() {
        let mut value = String::new();
        let chars = line.chars();
        for (x, c) in chars.enumerate() {
            if !c.is_ascii_digit() && !value.is_empty() {
                numbers.push(Number {
                    value: value.parse().unwrap(),
                    x: x as i32 - value.len() as i32,
                    y: y as i32,
                });
                value.clear();
            } else if c.is_ascii_digit() {
                value.push(c);
            }
            if c != '.' && !c.is_ascii_digit() {
                symbols.push(Symbol { value: c, x: x as i32, y: y as i32 })
            }
        }
        if !value.is_empty() {
            numbers.push(Number {
                value: value.parse().unwrap(),
                x: line.len() as i32 - value.len() as i32,
                y: y as i32,
            })
        }
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
        !(pos.y < self.y - 1 || pos.y > self.y + 1 
            || pos.x < self.x - 1 || pos.x > self.x + self.value.to_string().len() as i32)
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
