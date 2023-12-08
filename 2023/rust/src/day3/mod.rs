use std::{
    fs::File,
    io::{self, BufReader},
    rc::Rc,
    vec,
};

use nom::{
    branch::alt,
    bytes::complete::is_not,
    character::complete::digit1,
    combinator::{iterator, map},
};
use nom_locate::LocatedSpan;

use crate::utils;

type Span<'a> = LocatedSpan<&'a str>;

pub fn part_1() {
    if let Ok(lines) = utils::read_lines("./puzzle_inputs/day3.txt") {
        let mut valid_part_numbers: Vec<i32> = vec![];

        let schematic = Rc::new(parse_schematic(lines));

        for (row_index, row) in schematic.clone().iter().enumerate() {
            for element in row {
                if let ElementKind::Number(value) = &element.kind {
                    if row.iter().any(|other_element_in_row| {
                        is_adjacent_symbol_horizontal(element, other_element_in_row)
                    }) {
                        if let Ok(part_number) = value.iter().collect::<String>().parse::<i32>() {
                            valid_part_numbers.push(part_number);
                        }

                        continue;
                    }

                    if row_index == 0 {
                        if let Some(next_row) = schematic.clone().iter().nth(row_index + 1) {
                            if next_row.iter().any(|element_in_next_row| {
                                is_adjacent_symbol_vertical(element, element_in_next_row)
                            }) {
                                if let Ok(part_number) =
                                    value.iter().collect::<String>().parse::<i32>()
                                {
                                    valid_part_numbers.push(part_number);
                                }

                                continue;
                            }
                        }
                    } else {
                        if let Some(previous_row) = schematic.clone().iter().nth(row_index - 1) {
                            if previous_row.iter().any(|element_in_previous_row| {
                                is_adjacent_symbol_vertical(element, element_in_previous_row)
                            }) {
                                if let Ok(part_number) =
                                    value.iter().collect::<String>().parse::<i32>()
                                {
                                    valid_part_numbers.push(part_number);

                                    continue;
                                }
                            }
                        }

                        if let Some(next_row) = schematic.clone().iter().nth(row_index + 1) {
                            if next_row.iter().any(|element_in_next_row| {
                                is_adjacent_symbol_vertical(element, element_in_next_row)
                            }) {
                                if let Ok(part_number) =
                                    value.iter().collect::<String>().parse::<i32>()
                                {
                                    valid_part_numbers.push(part_number);

                                    continue;
                                }
                            }
                        }
                    }
                }
            }
        }

        println!(
            "Day 3 Part 1 solution: {}",
            valid_part_numbers.iter().sum::<i32>()
        );
    }
}

pub fn part_2() {
    if let Ok(lines) = utils::read_lines("./puzzle_inputs/day3.txt") {
        let mut gear_ratios: Vec<i32> = vec![];

        let schematic = Rc::new(parse_schematic(lines));

        for (row_index, row) in schematic.clone().iter().enumerate() {
            for element in row {
                if let ElementKind::Symbol('*') = element.kind {
                    let mut adjacent_part_numbers: Vec<i32> = vec![];

                    row.iter()
                        .filter(|other| is_adjacent_symbol_horizontal(other, element))
                        .for_each(|other| {
                            if let ElementKind::Number(value) = &other.kind {
                                if let Ok(part_number) =
                                    value.iter().collect::<String>().parse::<i32>()
                                {
                                    adjacent_part_numbers.push(part_number);
                                }
                            }
                        });

                    if row_index == 0 {
                        if let Some(next_row) = schematic.clone().iter().nth(row_index + 1) {
                            next_row
                                .iter()
                                .filter(|other| is_adjacent_symbol_vertical(other, element))
                                .for_each(|other| {
                                    if let ElementKind::Number(value) = &other.kind {
                                        if let Ok(part_number) =
                                            value.iter().collect::<String>().parse::<i32>()
                                        {
                                            adjacent_part_numbers.push(part_number);
                                        }
                                    }
                                });
                        }
                    } else {
                        if let Some(previous_row) = schematic.clone().iter().nth(row_index - 1) {
                            previous_row
                                .iter()
                                .filter(|other| is_adjacent_symbol_vertical(other, element))
                                .for_each(|other| {
                                    if let ElementKind::Number(value) = &other.kind {
                                        if let Ok(part_number) =
                                            value.iter().collect::<String>().parse::<i32>()
                                        {
                                            adjacent_part_numbers.push(part_number);
                                        }
                                    }
                                });
                        }

                        if let Some(next_row) = schematic.clone().iter().nth(row_index + 1) {
                            next_row
                                .iter()
                                .filter(|other| is_adjacent_symbol_vertical(other, element))
                                .for_each(|other| {
                                    if let ElementKind::Number(value) = &other.kind {
                                        if let Ok(part_number) =
                                            value.iter().collect::<String>().parse::<i32>()
                                        {
                                            adjacent_part_numbers.push(part_number);
                                        }
                                    }
                                });
                        }
                    }

                    if adjacent_part_numbers.len() != 2 {
                        continue;
                    } else {
                        gear_ratios.push(adjacent_part_numbers[0] * adjacent_part_numbers[1]);
                    }
                }
            }
        }

        println!("Day 3 Part 2 solution: {}", gear_ratios.iter().sum::<i32>());
    }
}

fn is_adjacent_symbol_horizontal(element: &Element, other_element: &Element) -> bool {
    if element.position == other_element.position {
        return false;
    }

    match &element.kind {
        ElementKind::Symbol(_) => false,
        ElementKind::Number(value) => match other_element.kind {
            ElementKind::Symbol(_) => {
                other_element.position == element.position - 1
                    || other_element.position == element.position + value.len() as i32
            }
            ElementKind::Number(_) => false,
        },
    }
}

fn is_adjacent_symbol_vertical(element: &Element, other_element: &Element) -> bool {
    match &element.kind {
        ElementKind::Symbol(_) => false,
        ElementKind::Number(value) => match other_element.kind {
            ElementKind::Symbol(_) => {
                other_element.position >= element.position - 1
                    && other_element.position <= (element.position + value.len() as i32)
            }
            ElementKind::Number(_) => false,
        },
    }
}

fn parse_schematic(lines: io::Lines<BufReader<File>>) -> Vec<Vec<Element>> {
    let mut elements: Vec<Vec<Element>> = vec![];

    for line in lines.flatten() {
        let parsed: Vec<Element> = iterator(
            Span::new(&line),
            alt((
                map(digit1::<Span, nom::error::Error<Span>>, |span| Element {
                    kind: ElementKind::Number(span.chars().collect()),
                    position: span.get_column() as i32,
                }),
                map(is_not(".0123456789"), |span: Span| Element {
                    kind: ElementKind::Symbol(span.chars().next().unwrap()),
                    position: span.get_column() as i32,
                }),
            )),
        )
        .collect();

        elements.push(parsed);
    }

    elements
}

#[derive(Debug)]
enum ElementKind {
    Symbol(char),
    Number(Vec<char>),
}

#[derive(Debug)]
struct Element {
    kind: ElementKind,
    position: i32,
}
