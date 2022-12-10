use std::fs::File;
use std::io::{self, BufRead};

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

    if let Ok(lines) = read_lines("./data/day1_part1.txt") {
        for line in lines {
            if let Ok(line_data) = line {
                if line_data == "" {
                    parsed_data.push(group);
                    group = vec![]
                } else if let Ok(line_value) = line_data.parse::<i32>() {
                    group.push(line_value);
                }
            }
        }
        if !group.is_empty() {
            parsed_data.push(group);
        }
    }

    parsed_data.iter().map(|elf| elf.iter().sum()).collect()
}

fn read_lines(filename: &str) -> io::Result<io::Lines<io::BufReader<File>>> {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
