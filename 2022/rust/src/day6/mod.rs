use std::collections::HashSet;

use crate::utils;

const PACKET_LENGTH: usize = 4;
const MESSAGE_LENGTH: usize = 14;

pub fn part_1() {
    let mut window_start = 0;
    let input = get_input();
    let mut current_window = input[window_start..window_start + PACKET_LENGTH].to_string();
    while !is_start_of_packet(&current_window) && input.len() >= window_start + PACKET_LENGTH + 2 {
        window_start += 1;
        current_window = input[window_start..window_start + PACKET_LENGTH].to_string();
    }

    println!("Day 6 part 1 solution: {}", window_start + PACKET_LENGTH);
}

pub fn part_2() {
    let mut window_start = 0;
    let input = get_input();
    let mut current_window = input[window_start..window_start + MESSAGE_LENGTH].to_string();
    while !is_start_of_message(&current_window) && input.len() >= window_start + MESSAGE_LENGTH + 2
    {
        window_start += 1;
        current_window = input[window_start..window_start + MESSAGE_LENGTH].to_string();
    }

    println!("Day 6 part 2 solution: {}", window_start + MESSAGE_LENGTH);
}

fn is_start_of_packet(window: &str) -> bool {
    let unique_letters_in_window: HashSet<char> = HashSet::from_iter(window.chars());
    unique_letters_in_window.len() == PACKET_LENGTH
}

fn is_start_of_message(window: &str) -> bool {
    let unique_letters_in_window: HashSet<char> = HashSet::from_iter(window.chars());
    unique_letters_in_window.len() == MESSAGE_LENGTH
}

fn get_input() -> String {
    let mut input = String::new();

    if let Ok(lines) = utils::read_lines("./data/day6.txt") {
        input = lines
            .flatten()
            .collect::<Vec<String>>()
            .first()
            .unwrap_or(&String::new())
            .trim()
            .to_string();
    }

    input
}
