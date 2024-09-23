advent_of_code::solution!(6);

pub fn part_one(input: &str) -> Option<u32> {
    let lines: Vec<&str> = input.lines().collect();
    let times: Vec<u32> = lines[0].split_whitespace().skip(1).map(|num| num.parse().unwrap()).collect();
    let targets: Vec<u32> = lines[1].split_whitespace().skip(1).map(|num| num.parse().unwrap()).collect();
    
    // let mut ways = vec![0; times.len()];
    let mut ways2 = Vec::new();
    for i in 0..times.len() {
        let time = times[i];
        let target = targets[i];
        // Simple iterative way
        // for t in 1..time {
        //     let dist = t * (time - t);
        //     if dist > target {
        //         ways[i] += 1;
        //     }
        // }

        // Compute where quadratic parabola is above target
        // solve for x in: y := x * (time - x) > target
        // then check if y at rounded x is greater or less than target
        // if greater round ways up, else round down
        let x: f32 = ((time as f32 / 2.0) + ((time as f32 / 2.0).powi(2) - target as f32).sqrt()).round();
        let y = x * (time as f32 - x);
        let v: f32 = (((time as f32)).powi(2) - 4.0 * target as f32).sqrt();
        if y as u32 > target {
            ways2.push(v.ceil() as u32)
        } else {
            ways2.push(v.floor() as u32)
        }
    }
    
    let res = ways2.iter().product();
    Some(res)
}

pub fn part_two(input: &str) -> Option<u64> {
    let lines: Vec<&str> = input.lines().collect();
    let time: u64 = lines[0].replace("Time:", "").replace(" ", "").parse().unwrap();
    let target: u64 = lines[1].replace("Distance:", "").replace(" ", "").parse().unwrap();

    // let mut ways = 0;
    //     for t in 1..time {
    //     let dist = t * (time - t);
    //     if dist > target {
    //         ways += 1;
    //     }
    // }

    let mut ways2 = 0;
    let x = ((time as f64 / 2.0) + ((time as f64 / 2.0).powi(2) - target as f64).sqrt()).round();
    let y = x * (time as f64 - x);
    let v = ((time as f64).powi(2) - 4.0 * target as f64).sqrt();
    if y as u64 > target {
        ways2 = v.ceil() as u64
    } else {
        ways2 = v.floor() as u64
    }
    
    Some(ways2)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(288));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(71503));
    }
}
