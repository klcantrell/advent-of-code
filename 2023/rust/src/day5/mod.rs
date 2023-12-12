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
use rayon::prelude::*;

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

    let mut min_location: i64 = i64::MAX;

    let (seeds, maps) = parse_almanac(input);

    let maps: [&Vec<AlmanacMap>; 7] = [
        maps.get(SEED_TO_SOIL_CATEGORY)
            .expect("seed-to-soil map incorrectly parsed"),
        maps.get(SOIL_TO_FERTILIZER_CATEGORY)
            .expect("soil-to-fertilizer map incorrectly parsed"),
        maps.get(FERTILIZER_TO_WATER_CATEGORY)
            .expect("fertilizer-to-water map incorrectly parsed"),
        maps.get(WATER_TO_LIGHT_CATEGORY)
            .expect("water-to-light map incorrectly parsed"),
        maps.get(LIGHT_TO_TEMPERATURE_CATEGORY)
            .expect("light-to-temperature map incorrectly parsed"),
        maps.get(TEMPERATURE_TO_HUMIDITY_CATEGORY)
            .expect("temperature-to-humidity map incorrectly parsed"),
        maps.get(HUMIDITY_TO_LOCATION_CATEGORY)
            .expect("humidity-to-location map incorrectly parsed"),
    ];

    for seed in seeds {
        min_location = i64::min(min_location, get_location(seed, maps));
    }

    println!("Day 5 Part 1 solution: {}", min_location);

    if let Ok(elapsed) = now.elapsed() {
        println!("👆 finished in {} seconds", elapsed.as_secs_f32())
    }
}

pub fn part_2() {
    let now = SystemTime::now();

    let input = include_str!("../../puzzle_inputs/day5.txt");

    let (seed_ranges, maps) = parse_almanac(input);

    let maps: [&Vec<AlmanacMap>; 7] = [
        maps.get(SEED_TO_SOIL_CATEGORY)
            .expect("seed-to-soil map incorrectly parsed"),
        maps.get(SOIL_TO_FERTILIZER_CATEGORY)
            .expect("soil-to-fertilizer map incorrectly parsed"),
        maps.get(FERTILIZER_TO_WATER_CATEGORY)
            .expect("fertilizer-to-water map incorrectly parsed"),
        maps.get(WATER_TO_LIGHT_CATEGORY)
            .expect("water-to-light map incorrectly parsed"),
        maps.get(LIGHT_TO_TEMPERATURE_CATEGORY)
            .expect("light-to-temperature map incorrectly parsed"),
        maps.get(TEMPERATURE_TO_HUMIDITY_CATEGORY)
            .expect("temperature-to-humidity map incorrectly parsed"),
        maps.get(HUMIDITY_TO_LOCATION_CATEGORY)
            .expect("humidity-to-location map incorrectly parsed"),
    ];

    let seed_ranges = seed_ranges.par_chunks(2);
    let seed_ranges_length = seed_ranges.len();

    let mut chunked_locations: Vec<i64> = seed_ranges
        .enumerate()
        .map(|(seed_range_index, seed_range_data)| {
            if seed_range_data.first().is_none() || seed_range_data.get(1).is_none() {
                panic!("Each seed range should have a start value and a range length value");
            }
            let mut min_location: i64 = i64::MAX;

            let seed_range_start = seed_range_data[0];
            let seed_range_length = seed_range_data[1];
            let upper_bound = seed_range_start + seed_range_length;
            let seed_range = seed_range_start..upper_bound;

            println!(
                "Processing seed range {}-{} - on step {} / {}",
                seed_range_start,
                upper_bound,
                seed_range_index + 1,
                seed_ranges_length,
            );

            for seed_value in seed_range {
                min_location = i64::min(min_location, get_location(seed_value, maps));
            }

            min_location
        })
        .collect();

    chunked_locations.sort();

    println!(
        "Day 5 Part 2 solution: {}",
        chunked_locations.first().unwrap_or(&0)
    );

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
                                source_range_end: parsed[1] + parsed[2],
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

fn get_location(seed_value: i64, maps: [&Vec<AlmanacMap>; 7]) -> i64 {
    maps.iter()
        .fold(seed_value, |previous_mapped_value, mappings| {
            mappings
                .iter()
                .find_map(|mapping| {
                    if (mapping.source_range_start..mapping.source_range_end)
                        .contains(&previous_mapped_value)
                    {
                        let next_mapped_value = previous_mapped_value - mapping.source_range_start
                            + mapping.destination_range_start;
                        Some(next_mapped_value)
                    } else {
                        None
                    }
                })
                .unwrap_or(previous_mapped_value)
        })
}

#[derive(Debug)]
struct AlmanacMap {
    destination_range_start: i64,
    source_range_start: i64,
    source_range_end: i64,
}
