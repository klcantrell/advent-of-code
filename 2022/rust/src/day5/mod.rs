use regex::Regex;

use crate::utils;

enum ParseMode {
    Stack,
    Instructions,
}

#[derive(Debug)]
struct MoveInstruction {
    from: usize,
    to: usize,
    size: usize,
}

pub fn part_1() {
    let (mut stacks, instructions) = parse_stacks();

    for instruction in instructions.iter() {
        let from_stack = &mut stacks.get_mut(instruction.from - 1).unwrap();

        let items_to_move = &mut from_stack.split_off(from_stack.len() - instruction.size);
        items_to_move.reverse();

        let to_stack = &mut stacks.get_mut(instruction.to - 1).unwrap();
        to_stack.append(items_to_move);
    }

    let tops = stacks
        .iter()
        .map(|stack| stack.last().unwrap().to_owned())
        .collect::<Vec<String>>()
        .join("");

    println!("Day 5 Part 1 Solution: {}", tops);
}

pub fn part_2() {
    let (mut stacks, instructions) = parse_stacks();

    for instruction in instructions.iter() {
        let from_stack = &mut stacks.get_mut(instruction.from - 1).unwrap();

        let items_to_move = &mut from_stack.split_off(from_stack.len() - instruction.size);
        let to_stack = &mut stacks.get_mut(instruction.to - 1).unwrap();

        to_stack.append(items_to_move);
    }

    let tops = stacks
        .iter()
        .map(|stack| stack.last().unwrap().to_owned())
        .collect::<Vec<String>>()
        .join("");

    println!("Day 5 Part 2 Solution: {}", tops);
}

fn parse_stacks() -> (Vec<Vec<String>>, Vec<MoveInstruction>) {
    let mut parse_mode = ParseMode::Stack;

    let mut stacks: Vec<Vec<String>> = vec![];
    let mut instructions: Vec<MoveInstruction> = vec![];

    if let Ok(lines) = utils::read_lines("./data/day5.txt") {
        for line in lines.flatten() {
            match parse_mode {
                ParseMode::Stack => {
                    if line.chars().nth(1).and_then(|v| v.to_digit(10)).is_some() {
                        parse_mode = ParseMode::Instructions;
                        continue;
                    }
                    let stack_level = parse_stack_level(line);
                    for (i, entry) in stack_level.iter().enumerate() {
                        if i >= stacks.len() {
                            stacks.push(vec![]);
                        }

                        if entry.trim().is_empty() {
                            continue;
                        }

                        stacks[i].push(entry.to_string());
                    }
                }
                ParseMode::Instructions => {
                    if !line.starts_with("move") {
                        continue;
                    }

                    let delimiter_pattern = Regex::new(r"move | from | to ").unwrap();
                    let instruction_values = delimiter_pattern
                        .split(&line)
                        .filter(|v| !v.is_empty())
                        .collect::<Vec<&str>>();

                    let size = instruction_values[0].parse::<usize>().unwrap();
                    let from = instruction_values[1].parse::<usize>().unwrap();
                    let to = instruction_values[2].parse::<usize>().unwrap();

                    instructions.push(MoveInstruction { from, to, size });
                }
            }
        }
    }

    stacks.iter_mut().for_each(|stack| stack.reverse());

    (stacks, instructions)
}

fn parse_stack_level(level: String) -> Vec<String> {
    fn parse_stack_level_aux(level_aux: String, stack_level: Vec<String>) -> Vec<String> {
        if level_aux.is_empty() {
            return stack_level;
        }

        let entry = level_aux
            .chars()
            .take(3)
            .filter(|&c| !(c == '[' || c == ']'))
            .collect::<String>();
        if entry.is_empty() {
            stack_level
        } else {
            let next_level_chars = level_aux.chars().skip(4).collect::<String>();
            parse_stack_level_aux(next_level_chars, [stack_level, vec![entry]].concat())
        }
    }

    parse_stack_level_aux(level, vec![])
}
