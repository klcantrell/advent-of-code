use nom::{
    branch::alt,
    bytes::complete::{tag, take_till},
    character::complete::digit1,
    IResult,
};

use crate::utils;

pub fn part_1() {
    if let Ok(lines) = utils::read_lines("./puzzle_inputs/day1.txt") {
        let result: i32 = lines
            .flatten()
            .map(|line| {
                let parsed = digits_part_1(line.as_str());

                let mut parsed_characters = parsed.chars();
                let first = parsed_characters.next();
                if let (Some(first), Some(last)) = (first, parsed_characters.next_back().or(first))
                {
                    format!("{}{}", first, last).parse::<i32>().unwrap_or(0)
                } else {
                    0
                }
            })
            .sum();

        println!("Day 1 Part 1 solution: {}", result);
    }
}

pub fn part_2() {
    if let Ok(lines) = utils::read_lines("./puzzle_inputs/day1.txt") {
        let result: i32 = lines
            .flatten()
            .map(|line| {
                let mut parsed = digits_part_2(line.as_str());

                let mut parsed_characters = parsed.chars();
                let first = parsed_characters.next();
                if let (Some(first), Some(last)) = (first, parsed_characters.next_back().or(first))
                {
                    format!("{}{}", first, last).parse::<i32>().unwrap_or(0)
                } else {
                    0
                }
            })
            .sum();

        println!("Day 1 Part 2 solution: {}", result);
    }
}

fn is_digit(input: char) -> bool {
    let mut b = [0; 1];
    let input_encoded = input.encode_utf8(&mut b);

    digit1::<&str, nom::error::Error<&str>>(input_encoded).is_ok()
}

fn digits_part_1(input: &str) -> String {
    let mut parsed = String::new();
    let mut remaining = Some(input);

    while let Some(remaining_to_process) = remaining {
        if let Ok((next_remaining, digit)) = digit1::<_, nom::error::Error<_>>(remaining_to_process)
        {
            parsed.push_str(digit);
            remaining = Some(next_remaining);
        } else if let Ok((with_digits, _)) =
            take_till::<_, _, nom::error::Error<_>>(is_digit)(remaining_to_process)
        {
            if with_digits.is_empty() {
                remaining = None;
            } else {
                remaining = Some(with_digits);
            }
        } else {
            remaining = None
        }
    }

    parsed
}

fn word_digit(input: &str) -> IResult<&str, &str> {
    alt((
        tag::<_, &str, nom::error::Error<_>>("one"),
        tag("two"),
        tag("three"),
        tag("four"),
        tag("five"),
        tag("six"),
        tag("seven"),
        tag("eight"),
        tag("nine"),
        tag("nine"),
    ))(input)
}

fn digits_part_2(input: &str) -> String {
    if let Ok((remaining, matched)) = word_digit(input) {}

    String::new()
}
