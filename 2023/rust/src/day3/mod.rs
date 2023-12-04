use std::{
    fs::File,
    io::{self, BufReader},
    rc::Rc,
    vec,
};

use crate::utils;

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
        let mut current_row: Vec<Element> = vec![];
        let mut digits: Vec<char> = vec![];

        for (column_index, character) in line.chars().enumerate() {
            if character.is_ascii_digit() {
                digits.push(character);

                if column_index == line.len() - 1 {
                    let position = column_index - digits.len() + 1;
                    current_row.push(Element {
                        kind: ElementKind::Number(digits),
                        position: position as i32,
                    });
                    digits = vec![];
                }
            } else {
                if !digits.is_empty() {
                    let position = column_index - digits.len();
                    current_row.push(Element {
                        kind: ElementKind::Number(digits),
                        position: position as i32,
                    });
                    digits = vec![];
                }

                if character == '.' {
                    continue;
                } else {
                    current_row.push(Element {
                        kind: ElementKind::Symbol(character),
                        position: column_index as i32,
                    });
                }
            }
        }

        elements.push(current_row);
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
