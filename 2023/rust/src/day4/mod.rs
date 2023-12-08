use std::{collections::HashMap, time::SystemTime};

use nom::{
    bytes::complete::tag,
    character::complete::{self, digit1, space1},
    multi::{many1, separated_list1},
    sequence::{delimited, preceded, terminated},
};

use crate::utils;

pub fn part_1() {
    let now = SystemTime::now();

    if let Ok(lines) = utils::read_lines("./puzzle_inputs/day4.txt") {
        let cards: Vec<Card> = lines.flatten().filter_map(parse_card).collect();

        let points = cards.iter().map(|card| {
            let mut points = 0;

            for player_value in &card.player_values {
                if card.winning_values.contains(player_value) {
                    if points == 0 {
                        points = 1;
                    } else {
                        points *= 2;
                    }
                }
            }

            points
        });

        println!("Day 4 Part 1 solution: {}", points.sum::<i32>());
    }

    if let Ok(elapsed) = now.elapsed() {
        println!("👆 finished in {} seconds", elapsed.as_secs_f32())
    }
}

pub fn part_2() {
    let now = SystemTime::now();

    if let Ok(lines) = utils::read_lines("./puzzle_inputs/day4.txt") {
        let cards: Vec<Card> = lines.flatten().filter_map(parse_card).collect();
        let cards_length = cards.len() as i32;

        let mut copies: HashMap<String, i32> = HashMap::new();

        for card in cards {
            let card_name_value = card
                .name
                .parse::<i32>()
                .expect("Card name should be numerical");
            let copies_of_this_card = *copies.get(&card_name_value.to_string()).unwrap_or(&0);

            let mut number_of_matches: i32 = 0;
            for player_value in &card.player_values {
                if card.winning_values.contains(player_value) {
                    number_of_matches += 1;
                }
            }

            for i in (card_name_value + 1)..=(card_name_value + number_of_matches) {
                let key = i.to_string();
                let mut previous_number_of_copies: i32 = 0;

                if let Some(previous_value) = copies.get(&key) {
                    previous_number_of_copies = *previous_value;
                }

                copies.insert(key, previous_number_of_copies + copies_of_this_card + 1);
            }
        }

        println!(
            "Day 4 Part 2 solution: {}",
            cards_length + copies.into_values().sum::<i32>()
        );
    }

    if let Ok(elapsed) = now.elapsed() {
        println!("👆 finished in {} seconds", elapsed.as_secs_f32())
    }
}

fn parse_card(line: String) -> Option<Card> {
    if let Ok((card_values_input, game_name)) = delimited(
        tag("Card"),
        preceded(space1, digit1::<&str, nom::error::Error<&str>>),
        terminated(complete::char(':'), many1(complete::char(' '))),
    )(&line)
    {
        if let Ok((_, card_values)) = separated_list1(
            delimited(
                many1(complete::char(' ')),
                complete::char('|'),
                many1(complete::char(' ')),
            ),
            separated_list1(
                many1(complete::char(' ')),
                digit1::<&str, nom::error::Error<&str>>,
            ),
        )(card_values_input)
        {
            if card_values.len() != 2 {
                panic!("The input is supposed to have 2 lists: 1 of winning values and 1 of player values.");
            }

            Some(Card {
                name: game_name.to_string(),
                winning_values: card_values[0]
                    .iter()
                    .filter_map(|v| v.parse::<i32>().ok())
                    .collect(),
                player_values: card_values[1]
                    .iter()
                    .filter_map(|v| v.parse::<i32>().ok())
                    .collect(),
            })
        } else {
            None
        }
    } else {
        None
    }
}

#[derive(Debug)]
struct Card {
    name: String,
    winning_values: Vec<i32>,
    player_values: Vec<i32>,
}
