use nom::{
    branch::alt,
    bytes::complete::{tag, take_till},
    character::complete::{digit1, space1},
    combinator::{iterator, map_res},
    multi::separated_list1,
    sequence::terminated,
    Parser,
};

pub fn part_1() {
    let input = include_str!("../../puzzle_inputs/day6.txt");

    let (times, distances) = parse_races_part_1(input);

    if times.len() != distances.len() {
        panic!("Each time entry should also have a distance entry");
    }

    let ways_to_win_per_race = times.iter().enumerate().map(|(index, time)| {
        let mut ways_to_win_count = 0;

        for time_held in 0..=*time {
            let distance_moved = time_held * (time - time_held);
            if distance_moved > distances[index] {
                ways_to_win_count += 1;
            }
        }

        ways_to_win_count
    });

    println!(
        "Day 6 Part 1 solution: {}",
        ways_to_win_per_race
            .reduce(|result, item| item * result)
            .unwrap_or_default()
    );
}

pub fn part_2() {
    let input = include_str!("../../puzzle_inputs/day6.txt");

    let (time, distance) = parse_races_part_2(input);

    let mut ways_to_win_count = 0;

    for time_held in 0..=time {
        let distance_moved = time_held * (time - time_held);
        if distance_moved > distance {
            ways_to_win_count += 1;
        }
    }

    println!("Day 6 Part 2 solution: {}", ways_to_win_count);
}

fn parse_races_part_1(input: &str) -> (Vec<i32>, Vec<i32>) {
    let mut times: Vec<i32> = vec![];
    let mut distances: Vec<i32> = vec![];

    let mut parsed = iterator(
        input,
        alt((
            terminated(
                tag::<&str, &str, nom::error::Error<&str>>("Time"),
                take_till(|c: char| c.is_ascii_digit()),
            )
            .and(map_res(
                terminated(
                    separated_list1(space1, digit1::<&str, nom::error::Error<&str>>),
                    take_till(|c: char| c.is_alphabetic()),
                ),
                |r| {
                    Ok::<Vec<i32>, nom::error::Error<&str>>(
                        r.iter()
                            .map(|x| x.parse::<i32>().unwrap_or_default())
                            .collect(),
                    )
                },
            )),
            terminated(
                tag::<&str, &str, nom::error::Error<&str>>("Distance"),
                take_till(|c: char| c.is_ascii_digit()),
            )
            .and(map_res(
                terminated(
                    separated_list1(space1, digit1::<&str, nom::error::Error<&str>>),
                    take_till(|c: char| c.is_alphabetic()),
                ),
                |r| {
                    Ok::<Vec<i32>, nom::error::Error<&str>>(
                        r.iter()
                            .map(|x| x.parse::<i32>().unwrap_or_default())
                            .collect(),
                    )
                },
            )),
        )),
    );

    parsed.for_each(|(section_name, section_data)| match section_name {
        "Time" => times = section_data,
        "Distance" => distances = section_data,
        _ => (),
    });

    (times, distances)
}

fn parse_races_part_2(input: &str) -> (i64, i64) {
    let mut time: i64 = 0;
    let mut distance: i64 = 0;

    let mut parsed = iterator(
        input,
        alt((
            terminated(
                tag::<&str, &str, nom::error::Error<&str>>("Time"),
                take_till(|c: char| c.is_ascii_digit()),
            )
            .and(map_res(
                terminated(
                    separated_list1(space1, digit1::<&str, nom::error::Error<&str>>),
                    take_till(|c: char| c.is_alphabetic()),
                ),
                |r| {
                    Ok::<i64, nom::error::Error<&str>>(
                        r.join("").parse::<i64>().unwrap_or_default(),
                    )
                },
            )),
            terminated(
                tag::<&str, &str, nom::error::Error<&str>>("Distance"),
                take_till(|c: char| c.is_ascii_digit()),
            )
            .and(map_res(
                terminated(
                    separated_list1(space1, digit1::<&str, nom::error::Error<&str>>),
                    take_till(|c: char| c.is_alphabetic()),
                ),
                |r| {
                    Ok::<i64, nom::error::Error<&str>>(
                        r.join("").parse::<i64>().unwrap_or_default(),
                    )
                },
            )),
        )),
    );

    parsed.for_each(|(section_name, section_data)| match section_name {
        "Time" => time = section_data,
        "Distance" => distance = section_data,
        _ => (),
    });

    (time, distance)
}
