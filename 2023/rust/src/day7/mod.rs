use std::{
    cmp::{self, Ordering},
    collections::HashMap,
};

use nom::{
    character::complete::{alphanumeric1, digit1, line_ending, space1},
    combinator::map_res,
    multi::separated_list1,
    sequence::separated_pair,
};

pub fn part_1() {
    let input = include_str!("../../puzzle_inputs/day7.txt");

    if let Some(mut hands) = parse_hands(input) {
        hands.sort_by(|a, b| a.0.cmp(&b.0));

        let total_winnings: i32 =
            hands
                .iter()
                .enumerate()
                .fold(0, |total_winnings_so_far, (rank, (_, bid))| {
                    total_winnings_so_far + ((rank as i32 + 1) * bid)
                });
        println!("Day 7 Part 1 solution: {}", total_winnings);
    }
}

fn parse_hands(input: &str) -> Option<Vec<(HandType, i32)>> {
    separated_list1(
        line_ending,
        separated_pair(
            map_res(alphanumeric1::<&str, nom::error::Error<&str>>, |raw_hand| {
                let mut char_map = HashMap::<char, i32>::new();
                raw_hand.chars().for_each(|c| {
                    if let Some(existing_char_count) = char_map.get(&c) {
                        char_map.insert(c, existing_char_count + 1);
                    } else {
                        char_map.insert(c, 1);
                    }
                });

                let mut char_map_values: Vec<i32> = char_map.into_values().collect();
                char_map_values.sort_by(|a, b| b.cmp(a));

                if char_map_values.contains(&5) {
                    Ok::<HandType, nom::error::Error<&str>>(HandType::FiveOfAKind(
                        raw_hand.to_string(),
                    ))
                } else if char_map_values.contains(&4) {
                    Ok(HandType::FourOfAKind(raw_hand.to_string()))
                } else if char_map_values.contains(&3) && char_map_values.contains(&2) {
                    Ok(HandType::FullHouse(raw_hand.to_string()))
                } else if char_map_values.contains(&3) {
                    Ok(HandType::ThreeOfAKind(raw_hand.to_string()))
                } else if char_map_values.len() == 3
                    && char_map_values[0] == 2
                    && char_map_values[1] == 2
                {
                    Ok(HandType::TwoPair(raw_hand.to_string()))
                } else if char_map_values.contains(&2) {
                    Ok(HandType::OnePair(raw_hand.to_string()))
                } else {
                    Ok(HandType::HighCard(raw_hand.to_string()))
                }
            }),
            space1,
            map_res(digit1, |raw_bid_value: &str| {
                Ok::<i32, nom::error::Error<&str>>(
                    raw_bid_value
                        .parse::<i32>()
                        .expect("Each hand's bid should be a number"),
                )
            }),
        ),
    )(input)
    .map(|(_, parsed)| parsed)
    .ok()
}

#[derive(Debug, Eq)]
enum HandType {
    FiveOfAKind(String),
    FourOfAKind(String),
    FullHouse(String),
    ThreeOfAKind(String),
    TwoPair(String),
    OnePair(String),
    HighCard(String),
}

impl Ord for HandType {
    fn cmp(&self, other: &Self) -> Ordering {
        match (self, other) {
            (HandType::FiveOfAKind(left), HandType::FiveOfAKind(right)) => {
                compare_hands_card_by_card(left, right)
            }
            (HandType::FiveOfAKind(_), _) => Ordering::Greater,
            (HandType::FourOfAKind(_), HandType::FiveOfAKind(_)) => Ordering::Less,
            (HandType::FourOfAKind(left), HandType::FourOfAKind(right)) => {
                compare_hands_card_by_card(left, right)
            }
            (HandType::FourOfAKind(_), _) => Ordering::Greater,
            (HandType::FullHouse(_), HandType::FiveOfAKind(_)) => Ordering::Less,
            (HandType::FullHouse(_), HandType::FourOfAKind(_)) => Ordering::Less,
            (HandType::FullHouse(left), HandType::FullHouse(right)) => {
                compare_hands_card_by_card(left, right)
            }
            (HandType::FullHouse(_), _) => Ordering::Greater,
            (HandType::ThreeOfAKind(_), HandType::FiveOfAKind(_)) => Ordering::Less,
            (HandType::ThreeOfAKind(_), HandType::FourOfAKind(_)) => Ordering::Less,
            (HandType::ThreeOfAKind(_), HandType::FullHouse(_)) => Ordering::Less,
            (HandType::ThreeOfAKind(left), HandType::ThreeOfAKind(right)) => {
                compare_hands_card_by_card(left, right)
            }
            (HandType::ThreeOfAKind(_), _) => Ordering::Greater,
            (HandType::TwoPair(_), HandType::FiveOfAKind(_)) => Ordering::Less,
            (HandType::TwoPair(_), HandType::FourOfAKind(_)) => Ordering::Less,
            (HandType::TwoPair(_), HandType::FullHouse(_)) => Ordering::Less,
            (HandType::TwoPair(_), HandType::ThreeOfAKind(_)) => Ordering::Less,
            (HandType::TwoPair(left), HandType::TwoPair(right)) => {
                compare_hands_card_by_card(left, right)
            }
            (HandType::TwoPair(_), _) => Ordering::Greater,
            (HandType::OnePair(_), HandType::FiveOfAKind(_)) => Ordering::Less,
            (HandType::OnePair(_), HandType::FourOfAKind(_)) => Ordering::Less,
            (HandType::OnePair(_), HandType::FullHouse(_)) => Ordering::Less,
            (HandType::OnePair(_), HandType::ThreeOfAKind(_)) => Ordering::Less,
            (HandType::OnePair(_), HandType::TwoPair(_)) => Ordering::Less,
            (HandType::OnePair(left), HandType::OnePair(right)) => {
                compare_hands_card_by_card(left, right)
            }
            (HandType::OnePair(_), HandType::HighCard(_)) => Ordering::Greater,
            (HandType::HighCard(left), HandType::HighCard(right)) => {
                compare_hands_card_by_card(left, right)
            }
            (HandType::HighCard(_), _) => Ordering::Less,
        }
    }
}

impl PartialOrd for HandType {
    fn partial_cmp(&self, other: &Self) -> Option<cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for HandType {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::FiveOfAKind(left), Self::FiveOfAKind(right)) => left == right,
            (Self::FourOfAKind(left), Self::FourOfAKind(right)) => left == right,
            (Self::FullHouse(left), Self::FullHouse(right)) => left == right,
            (Self::ThreeOfAKind(left), Self::ThreeOfAKind(right)) => left == right,
            (Self::TwoPair(left), Self::TwoPair(right)) => left == right,
            (Self::OnePair(left), Self::OnePair(right)) => left == right,
            (Self::HighCard(left), Self::HighCard(right)) => left == right,
            _ => false,
        }
    }
}

fn compare_hands_card_by_card(left: &str, right: &str) -> Ordering {
    left.chars()
        .zip(right.chars())
        .fold(Ordering::Equal, |ordering, (left_char, right_char)| {
            if let Ordering::Equal = ordering {
                if let (Some(left_num), Some(right_num)) =
                    (left_char.to_digit(10), right_char.to_digit(10))
                {
                    left_num.cmp(&right_num)
                } else if left_char.is_numeric() {
                    Ordering::Less
                } else if right_char.is_numeric() {
                    Ordering::Greater
                } else {
                    match (left_char, right_char) {
                        ('A', 'A') => Ordering::Equal,
                        ('A', _) => Ordering::Greater,
                        ('K', 'A') => Ordering::Less,
                        ('K', 'K') => Ordering::Equal,
                        ('K', _) => Ordering::Greater,
                        ('Q', 'A') => Ordering::Less,
                        ('Q', 'K') => Ordering::Less,
                        ('Q', 'Q') => Ordering::Equal,
                        ('Q', _) => Ordering::Greater,
                        ('J', 'A') => Ordering::Less,
                        ('J', 'K') => Ordering::Less,
                        ('J', 'Q') => Ordering::Less,
                        ('J', 'J') => Ordering::Equal,
                        ('J', _) => Ordering::Greater,
                        ('T', 'T') => Ordering::Equal,
                        ('T', _) => Ordering::Less,
                        _ => panic!("Invalid card. Lettered cards should be A, J, Q, J or T"),
                    }
                }
            } else {
                ordering
            }
        })
}
