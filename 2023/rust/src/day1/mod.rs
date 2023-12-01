use crate::utils;

pub fn part_1() {
    if let Ok(lines) = utils::read_lines("./puzzle_inputs/day1.txt") {
        let result: i32 = lines
            .flatten()
            .map(|line| {
                let digits: Vec<i32> = line
                    .split("")
                    .filter_map(|character| character.parse::<i32>().ok())
                    .collect();

                if let (Some(first), Some(last)) = (digits.first(), digits.last()) {
                    if let Ok(result) = format!("{}{}", first, last).parse::<i32>() {
                        result
                    } else {
                        0
                    }
                } else {
                    0
                }
            })
            .sum();

        println!("Day 1 Part 1 solution: {}", result)
    }
}
