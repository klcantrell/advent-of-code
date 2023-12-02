use nom::{bytes::complete::take_till, character::complete::digit1, IResult};

use crate::utils;

pub fn part_1() {
    if let Ok(lines) = utils::read_lines("./puzzle_inputs/day1.txt") {
        let result: i32 = lines
            .flatten()
            .map(|line| {
                let mut remaining = Some(line.as_str());
                let mut parsed = String::new();

                while let Some(remaining_to_process) = remaining {
                    if let Ok((next_remaining, digit)) =
                        digit1::<&str, nom::error::Error<&str>>(remaining_to_process)
                    {
                        parsed.push_str(digit);
                        remaining = Some(next_remaining);
                    } else if let Ok((with_digits, _)) = until_is_digit(remaining_to_process) {
                        if with_digits.is_empty() {
                            remaining = None;
                        } else {
                            remaining = Some(with_digits);
                        }
                    } else {
                        remaining = None
                    }
                }

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

fn is_digit(input: char) -> bool {
    let mut b = [0; 1];
    let input_encoded = input.encode_utf8(&mut b);

    digit1::<&str, nom::error::Error<&str>>(input_encoded).is_ok()
}

fn until_is_digit(input: &str) -> IResult<&str, &str> {
    take_till(is_digit)(input)
}
