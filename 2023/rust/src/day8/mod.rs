use std::collections::HashMap;

use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{alpha1, alphanumeric1},
    combinator::{eof, iterator},
    sequence::{delimited, separated_pair, terminated},
    IResult,
};

type DesertMap<'a> = (&'a str, HashMap<String, (String, String)>);

pub fn part_1() {
    let input = include_str!("../../puzzle_inputs/day8.txt");
    let (_, (directions, map)) = parse_desert_map(input).expect("Malformed desert map");

    let mut step_count = 0;
    let mut current_step = "AAA";

    while current_step != "ZZZ" {
        let direction = directions
            .chars()
            .nth(step_count % directions.len())
            .expect("Not enough steps in puzzle input");

        let crossroad = map
            .get(current_step)
            .unwrap_or_else(|| panic!("{} should be a valid destination", current_step));
        current_step = if direction == 'L' {
            &crossroad.0
        } else {
            &crossroad.1
        };

        step_count += 1;
    }

    println!("Day 8 Part 1 solution: {}", step_count);
}

pub fn part_2() {
    let input = include_str!("../../puzzle_inputs/day8.txt");
    let (_, (directions, map)) = parse_desert_map(input).expect("Malformed desert map");

    let ghost_nodes: Vec<String> = map
        .iter()
        .filter_map(|(node, _)| {
            if node.ends_with('A') {
                Some(node.to_string())
            } else {
                None
            }
        })
        .collect();

    let initial_destinations: Vec<usize> = ghost_nodes
        .iter()
        .map(|node| {
            let mut step_count = 0;
            let mut current_step = node;

            while !current_step.ends_with('Z') {
                let direction = directions
                    .chars()
                    .nth(step_count % directions.len())
                    .expect("Not enough steps in puzzle input");

                let crossroad = map
                    .get(current_step)
                    .unwrap_or_else(|| panic!("{} should be a valid destination", current_step));
                current_step = if direction == 'L' {
                    &crossroad.0
                } else {
                    &crossroad.1
                };

                step_count += 1;
            }

            step_count
        })
        .collect();

    let converged_destination = initial_destinations
        .iter()
        .fold(1, |d, current| num::integer::lcm(d, *current));

    println!("Day 8 Part 2 solution: {}", converged_destination);
}

fn parse_desert_map(input: &str) -> IResult<&str, DesertMap> {
    let (map_input, directions) = terminated(alpha1, tag("\n\n"))(input)?;
    let mut map: HashMap<String, (String, String)> = HashMap::new();

    let mut map_rows = iterator(
        map_input,
        terminated(
            separated_pair(
                alphanumeric1::<&str, nom::error::Error<&str>>,
                tag(" = "),
                delimited(
                    nom::character::complete::char('('),
                    separated_pair(alphanumeric1, tag(", "), alphanumeric1),
                    nom::character::complete::char(')'),
                ),
            ),
            alt((tag("\n"), eof)),
        ),
    );

    map_rows.for_each(|map_row| {
        map.insert(
            map_row.0.to_string(),
            (map_row.1 .0.to_string(), map_row.1 .1.to_string()),
        );
    });

    Ok(("", (directions, map)))
}
