use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::digit1,
    multi::separated_list1,
    sequence::{delimited, pair},
};

use crate::utils;

const RED_RULE: i32 = 12;
const GREEN_RULE: i32 = 13;
const BLUE_RULE: i32 = 14;

pub fn part_1() {
    if let Ok(lines) = utils::read_lines("./puzzle_inputs/day2.txt") {
        let games: Vec<Game> = lines
            .flatten()
            .filter_map(|mut line| {
                line.retain(|c| !c.is_whitespace());

                parse_game(line)
            })
            .collect();

        let qualifying_games = games.iter().filter(|game| {
            game.rounds.iter().all(|round| {
                round.red <= RED_RULE && round.blue <= BLUE_RULE && round.green <= GREEN_RULE
            })
        });

        println!(
            "Day 2 Part 1 solution: {}",
            qualifying_games.fold(0, |sum_of_ids, game| sum_of_ids + game.id)
        );
    }
}

pub fn part_2() {
    if let Ok(lines) = utils::read_lines("./puzzle_inputs/day2.txt") {
        let games: Vec<Game> = lines
            .flatten()
            .filter_map(|mut line| {
                line.retain(|c| !c.is_whitespace());

                parse_game(line)
            })
            .collect();

        let minimums = games.iter().map(|game| {
            game.rounds.iter().fold(
                (0, 0, 0),
                |(minimum_red, minimum_green, minimum_blue), round| {
                    let next_minimum_red = i32::max(minimum_red, round.red);
                    let next_minimum_green = i32::max(minimum_green, round.green);
                    let next_minimum_blue = i32::max(minimum_blue, round.blue);

                    (next_minimum_red, next_minimum_green, next_minimum_blue)
                },
            )
        });

        println!(
            "Day 2 Part 1 solution: {}",
            minimums.fold(
                0,
                |sum_of_powers, (minimum_red, minimum_green, minimum_blue)| minimum_red
                    * minimum_green
                    * minimum_blue
                    + sum_of_powers
            )
        );
    }
}

fn parse_game(line: String) -> Option<Game> {
    if let Ok((rounds_input, game_name)) = delimited(
        tag::<&str, &str, nom::error::Error<_>>("Game"),
        digit1,
        tag(":"),
    )(line.as_str())
    {
        if let Ok((_, rounds)) = separated_list1(
            tag(";"),
            separated_list1(
                tag(","),
                alt((
                    pair(digit1::<&str, nom::error::Error<&str>>, tag("red")),
                    pair(digit1, tag("blue")),
                    pair(digit1, tag("green")),
                )),
            ),
        )(rounds_input)
        {
            Some(Game::create(game_name.to_string(), rounds))
        } else {
            None
        }
    } else {
        None
    }
}

#[derive(Debug)]
struct Game {
    id: i32,
    rounds: Vec<Round>,
}

impl Game {
    fn create(name: String, parsed_rounds: Vec<Vec<(&str, &str)>>) -> Self {
        Self {
            id: name.parse::<i32>().unwrap_or(0),
            rounds: parsed_rounds
                .iter()
                .map(|round| {
                    round.iter().fold(
                        Round {
                            red: 0,
                            green: 0,
                            blue: 0,
                        },
                        |mut round_result, (color_value, color_name)| {
                            match *color_name {
                                "red" => round_result.red = color_value.parse::<i32>().unwrap_or(0),
                                "green" => {
                                    round_result.green = color_value.parse::<i32>().unwrap_or(0)
                                }
                                "blue" => {
                                    round_result.blue = color_value.parse::<i32>().unwrap_or(0)
                                }
                                _ => (),
                            };

                            round_result
                        },
                    )
                })
                .collect(),
        }
    }
}

#[derive(Debug)]
struct Round {
    red: i32,
    green: i32,
    blue: i32,
}
