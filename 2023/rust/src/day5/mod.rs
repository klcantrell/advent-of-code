use std::{collections::HashMap, time::SystemTime};

use nom::{
    branch::alt,
    bytes::complete::{tag, take_till, take_until, take_while},
    character::complete::{self, digit1, space1},
    combinator::{eof, iterator},
    multi::separated_list1,
    sequence::terminated,
    Parser,
};

const SEED_TO_SOIL_CATEGORY: &str = "seed-to-soil";
const SOIL_TO_FERTILIZER_CATEGORY: &str = "soil-to-fertilizer";
const FERTILIZER_TO_WATER_CATEGORY: &str = "fertilizer-to-water";
const WATER_TO_LIGHT_CATEGORY: &str = "water-to-light";
const LIGHT_TO_TEMPERATURE_CATEGORY: &str = "light-to-temperature";
const TEMPERATURE_TO_HUMIDITY_CATEGORY: &str = "temperature-to-humidity";
const HUMIDITY_TO_LOCATION_CATEGORY: &str = "humidity-to-location";

pub fn part_1() {
    let now = SystemTime::now();

    let input = include_str!("../../puzzle_inputs/day5.txt");

    let (seeds, maps) = parse_almanac(input);

    let mut locations: Vec<i64> = seeds.iter().map(move |seed| get_location(*seed, &maps)).collect();

    locations.sort();

    println!("Day 5 Part 1 solution: {}", locations.first().unwrap_or(&0));

    if let Ok(elapsed) = now.elapsed() {
        println!("👆 finished in {} seconds", elapsed.as_secs_f32())
    }
}

fn parse_almanac(input: &str) -> (Vec<i64>, HashMap<String, Vec<AlmanacMap>>) {
    let mut seeds: Vec<i64> = vec![];
    let mut maps = HashMap::<String, Vec<AlmanacMap>>::new();

    let mut first_parse = iterator(
        input,
        alt((
            terminated(
                tag::<&str, &str, nom::error::Error<&str>>("seeds"),
                take_till(|c: char| c.is_ascii_digit()),
            )
            .and(terminated(take_until("\n\n"), tag("\n\n")).map(|x: &str| x.trim())),
            terminated(
                tag::<&str, &str, nom::error::Error<&str>>(SEED_TO_SOIL_CATEGORY),
                take_till(|c: char| c.is_ascii_digit()),
            )
            .and(terminated(take_until("\n\n"), tag("\n\n")).map(|x: &str| x.trim())),
            terminated(
                tag::<&str, &str, nom::error::Error<&str>>(SOIL_TO_FERTILIZER_CATEGORY),
                take_till(|c: char| c.is_ascii_digit()),
            )
            .and(terminated(take_until("\n\n"), tag("\n\n")).map(|x: &str| x.trim())),
            terminated(
                tag::<&str, &str, nom::error::Error<&str>>(FERTILIZER_TO_WATER_CATEGORY),
                take_till(|c: char| c.is_ascii_digit()),
            )
            .and(terminated(take_until("\n\n"), tag("\n\n")).map(|x: &str| x.trim())),
            terminated(
                tag::<&str, &str, nom::error::Error<&str>>(WATER_TO_LIGHT_CATEGORY),
                take_till(|c: char| c.is_ascii_digit()),
            )
            .and(terminated(take_until("\n\n"), tag("\n\n")).map(|x: &str| x.trim())),
            terminated(
                tag::<&str, &str, nom::error::Error<&str>>(LIGHT_TO_TEMPERATURE_CATEGORY),
                take_till(|c: char| c.is_ascii_digit()),
            )
            .and(terminated(take_until("\n\n"), tag("\n\n")).map(|x: &str| x.trim())),
            terminated(
                tag::<&str, &str, nom::error::Error<&str>>(TEMPERATURE_TO_HUMIDITY_CATEGORY),
                take_till(|c: char| c.is_ascii_digit()),
            )
            .and(terminated(take_until("\n\n"), tag("\n\n")).map(|x: &str| x.trim())),
            terminated(
                tag::<&str, &str, nom::error::Error<&str>>(HUMIDITY_TO_LOCATION_CATEGORY),
                take_till(|c: char| c.is_ascii_digit()),
            )
            .and(terminated(
                take_while(|c: char| c.is_ascii_digit() || c.is_whitespace())
                    .map(|x: &str| x.trim()),
                eof,
            )),
        )),
    );

    for (section_name, section) in &mut first_parse {
        match section_name {
            "seeds" => {
                seeds = separated_list1(
                    space1,
                    digit1::<&str, nom::error::Error<&str>>
                        .map(|x: &str| x.parse::<i64>().unwrap_or_default()),
                )
                .parse(section)
                .map(|(_, result)| result)
                .unwrap_or_default()
            }
            SEED_TO_SOIL_CATEGORY
            | SOIL_TO_FERTILIZER_CATEGORY
            | FERTILIZER_TO_WATER_CATEGORY
            | WATER_TO_LIGHT_CATEGORY
            | LIGHT_TO_TEMPERATURE_CATEGORY
            | TEMPERATURE_TO_HUMIDITY_CATEGORY
            | HUMIDITY_TO_LOCATION_CATEGORY => {
                maps.insert(
                    section_name.into(),
                    separated_list1(
                        complete::char('\n'),
                        separated_list1(
                            space1,
                            digit1::<&str, nom::error::Error<&str>>
                                .map(|x: &str| x.parse::<i64>().unwrap_or_default()),
                        )
                        .map(|parsed| {
                            if parsed.len() != 3 {
                                panic!("Almanac map should contain 3 values");
                            }

                            AlmanacMap {
                                destination_range_start: parsed[0],
                                source_range_start: parsed[1],
                                range_length: parsed[2],
                            }
                        }),
                    )
                    .parse(section)
                    .map(|(_, parsed)| parsed)
                    .unwrap_or_default(),
                );
            }
            _ => (),
        }
    }

    (seeds, maps)
}

fn get_location(seed_value: i64, maps: &HashMap<String, Vec<AlmanacMap>>) -> i64 {
    let soil = maps
        .get(SEED_TO_SOIL_CATEGORY)
        .unwrap_or(&vec![])
        .iter()
        .find_map(|mapping| {
            let source_upper_bound = mapping.source_range_start + mapping.range_length;
            if (mapping.source_range_start..source_upper_bound).contains(&seed_value) {
                let desination_upper_bound = mapping.destination_range_start + mapping.range_length;
                let potential_destination =
                    seed_value - mapping.source_range_start + mapping.destination_range_start;
                if (mapping.destination_range_start..desination_upper_bound)
                    .contains(&potential_destination)
                {
                    Some(potential_destination)
                } else {
                    None
                }
            } else {
                None
            }
        })
        .unwrap_or(seed_value);

    let fertilizer = maps
        .get(SOIL_TO_FERTILIZER_CATEGORY)
        .unwrap_or(&vec![])
        .iter()
        .find_map(|mapping| {
            let source_upper_bound = mapping.source_range_start + mapping.range_length;
            if (mapping.source_range_start..source_upper_bound).contains(&soil) {
                let desination_upper_bound = mapping.destination_range_start + mapping.range_length;
                let potential_destination =
                    soil - mapping.source_range_start + mapping.destination_range_start;
                if (mapping.destination_range_start..desination_upper_bound)
                    .contains(&potential_destination)
                {
                    Some(potential_destination)
                } else {
                    None
                }
            } else {
                None
            }
        })
        .unwrap_or(soil);

    let water = maps
        .get(FERTILIZER_TO_WATER_CATEGORY)
        .unwrap_or(&vec![])
        .iter()
        .find_map(|mapping| {
            let source_upper_bound = mapping.source_range_start + mapping.range_length;
            if (mapping.source_range_start..source_upper_bound).contains(&fertilizer) {
                let desination_upper_bound = mapping.destination_range_start + mapping.range_length;
                let potential_destination =
                    fertilizer - mapping.source_range_start + mapping.destination_range_start;
                if (mapping.destination_range_start..desination_upper_bound)
                    .contains(&potential_destination)
                {
                    Some(potential_destination)
                } else {
                    None
                }
            } else {
                None
            }
        })
        .unwrap_or(fertilizer);

    let light = maps
        .get(WATER_TO_LIGHT_CATEGORY)
        .unwrap_or(&vec![])
        .iter()
        .find_map(|mapping| {
            let source_upper_bound = mapping.source_range_start + mapping.range_length;
            if (mapping.source_range_start..source_upper_bound).contains(&water) {
                let desination_upper_bound = mapping.destination_range_start + mapping.range_length;
                let potential_destination =
                    water - mapping.source_range_start + mapping.destination_range_start;
                if (mapping.destination_range_start..desination_upper_bound)
                    .contains(&potential_destination)
                {
                    Some(potential_destination)
                } else {
                    None
                }
            } else {
                None
            }
        })
        .unwrap_or(water);

    let temperature = maps
        .get(LIGHT_TO_TEMPERATURE_CATEGORY)
        .unwrap_or(&vec![])
        .iter()
        .find_map(|mapping| {
            let source_upper_bound = mapping.source_range_start + mapping.range_length;
            if (mapping.source_range_start..source_upper_bound).contains(&light) {
                let desination_upper_bound = mapping.destination_range_start + mapping.range_length;
                let potential_destination =
                    light - mapping.source_range_start + mapping.destination_range_start;
                if (mapping.destination_range_start..desination_upper_bound)
                    .contains(&potential_destination)
                {
                    Some(potential_destination)
                } else {
                    None
                }
            } else {
                None
            }
        })
        .unwrap_or(light);

    let humidity = maps
        .get(TEMPERATURE_TO_HUMIDITY_CATEGORY)
        .unwrap_or(&vec![])
        .iter()
        .find_map(|mapping| {
            let source_upper_bound = mapping.source_range_start + mapping.range_length;
            if (mapping.source_range_start..source_upper_bound).contains(&temperature) {
                let desination_upper_bound = mapping.destination_range_start + mapping.range_length;
                let potential_destination =
                    temperature - mapping.source_range_start + mapping.destination_range_start;
                if (mapping.destination_range_start..desination_upper_bound)
                    .contains(&potential_destination)
                {
                    Some(potential_destination)
                } else {
                    None
                }
            } else {
                None
            }
        })
        .unwrap_or(temperature);

    let location = maps
        .get(HUMIDITY_TO_LOCATION_CATEGORY)
        .unwrap_or(&vec![])
        .iter()
        .find_map(|mapping| {
            let source_upper_bound = mapping.source_range_start + mapping.range_length;
            if (mapping.source_range_start..source_upper_bound).contains(&humidity) {
                let desination_upper_bound = mapping.destination_range_start + mapping.range_length;
                let potential_destination =
                    humidity - mapping.source_range_start + mapping.destination_range_start;
                if (mapping.destination_range_start..desination_upper_bound)
                    .contains(&potential_destination)
                {
                    Some(potential_destination)
                } else {
                    None
                }
            } else {
                None
            }
        })
        .unwrap_or(humidity);

    location
}

#[derive(Debug)]
struct AlmanacMap {
    destination_range_start: i64,
    source_range_start: i64,
    range_length: i64,
}
