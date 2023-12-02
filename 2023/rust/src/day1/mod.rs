use nom::{
    branch::alt,
    bytes::complete::{tag, take_till},
    character::complete::digit1,
    combinator::map_res,
    IResult,
};

use crate::utils;

pub fn part_1() {
    if let Ok(lines) = utils::read_lines("./puzzle_inputs/day1.txt") {
        let result: i32 = lines
            .flatten()
            .map(|line| {
                let parsed = digits_part_1(line.as_str());
                extract_digits(parsed)
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
                let parsed = digits_part_2(line.as_str());
                extract_digits(parsed)
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

fn make_word_digit_tags<'a>() -> (
    impl Fn(&'a str) -> IResult<&str, &str>,
    impl Fn(&'a str) -> IResult<&str, &str>,
    impl Fn(&'a str) -> IResult<&str, &str>,
    impl Fn(&'a str) -> IResult<&str, &str>,
    impl Fn(&'a str) -> IResult<&str, &str>,
    impl Fn(&'a str) -> IResult<&str, &str>,
    impl Fn(&'a str) -> IResult<&str, &str>,
    impl Fn(&'a str) -> IResult<&str, &str>,
    impl Fn(&'a str) -> IResult<&str, &str>,
) {
    (
        tag("one"),
        tag("two"),
        tag("three"),
        tag("four"),
        tag("five"),
        tag("six"),
        tag("seven"),
        tag("eight"),
        tag("nine"),
    )
}

fn word_digit(input: &str) -> IResult<&str, &str> {
    map_res(alt(make_word_digit_tags()), |res| match res {
        "one" => Ok::<&str, nom::error::Error<&str>>("1"),
        "two" => Ok("2"),
        "three" => Ok("3"),
        "four" => Ok("4"),
        "five" => Ok("5"),
        "six" => Ok("6"),
        "seven" => Ok("7"),
        "eight" => Ok("8"),
        "nine" => Ok("9"),
        _ => Ok("0"),
    })(input)
}

fn digits_part_2(input: &str) -> String {
    let mut parsed = String::new();
    let mut remaining = Some(input);

    while let Some(remaining_to_process) = remaining {
        if let Ok((_, digit)) = word_digit(remaining_to_process) {
            parsed.push_str(digit);
            remaining = Some(&remaining_to_process[1..]);
        } else if let Ok((next_remaining, digit)) =
            digit1::<_, nom::error::Error<_>>(remaining_to_process)
        {
            parsed.push_str(digit);
            remaining = Some(next_remaining);
        } else if !remaining_to_process.is_empty() {
            remaining = Some(&remaining_to_process[1..]);
        } else {
            remaining = None;
        }
    }

    parsed
}

fn extract_digits(parsed: String) -> i32 {
    let mut parsed_characters = parsed.chars();
    let first = parsed_characters.next();
    if let (Some(first), Some(last)) = (first, parsed_characters.next_back().or(first)) {
        format!("{}{}", first, last).parse::<i32>().unwrap_or(0)
    } else {
        0
    }
}
