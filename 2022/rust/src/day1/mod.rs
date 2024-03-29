use crate::utils;

pub fn part_1() {
    println!(
        "Day 1 Part 1 soluton: {}",
        calories_by_elf().into_iter().max().unwrap_or(0)
    );
}

pub fn part_2() {
    let mut calories_by_elf = calories_by_elf();
    calories_by_elf.sort_by(|left, right| right.cmp(left));
    println!(
        "Day 1 Part 2 solution: {}",
        calories_by_elf.iter().take(3).sum::<i32>()
    );
}

fn calories_by_elf() -> Vec<i32> {
    let mut parsed_data: Vec<Vec<i32>> = vec![];
    let mut group: Vec<i32> = vec![];

    if let Ok(lines) = utils::read_lines("./data/day1.txt") {
        for line in lines.flatten() {
            if line.is_empty() {
                parsed_data.push(group);
                group = vec![]
            } else if let Ok(line_value) = line.parse::<i32>() {
                group.push(line_value);
            }
        }
        if !group.is_empty() {
            parsed_data.push(group);
        }
    }

    parsed_data.iter().map(|elf| elf.iter().sum()).collect()
}

