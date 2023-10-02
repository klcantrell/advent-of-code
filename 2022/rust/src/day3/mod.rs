use std::collections::{HashSet, HashMap};

use crate::utils;

pub fn part_1() {
    let common_items = lines_to_rucksacks()
        .iter()
        .map(find_common_item_type_in_rucksack)
        .map(|item_type| priority_of_item_type(&item_type))
        .collect::<Vec<i32>>();
    println!(
        "Day 3 Part 1 solution: {}",
        common_items.iter().sum::<i32>()
    );
}

pub fn part_2() {
    let common_items = lines_to_rucksacks()
        .iter()
        .map(|sack| sack.compartment_one.clone() + &sack.compartment_two)
        .collect::<Vec<String>>()
        .as_slice()
        .chunks(3)
        .map(|chunk| find_common_item_type_among_rucksacks(chunk.to_vec()))
        .map(|item_type| priority_of_item_type(&item_type))
        .collect::<Vec<i32>>();

    println!(
        "Day 3 Part 2 solution: {}",
        common_items.iter().sum::<i32>()
    );
}

fn lines_to_rucksacks() -> Vec<Rucksack> {
    let mut rucksacks: Vec<Rucksack> = vec![];

    if let Ok(lines) = utils::read_lines("./data/day3.txt") {
        for line in lines.flatten() {
            rucksacks.push(
                Rucksack {
                    compartment_one: line.chars().take(line.len() / 2).collect(),
                    compartment_two: line.chars().skip(line.len() / 2).collect(),
                }
            )
        }
    }

    rucksacks
}

fn find_common_item_type_in_rucksack(rucksack: &Rucksack) -> String {
    let compartment_one = HashSet::<char>::from_iter(rucksack.compartment_one.chars());
    let compartment_two = HashSet::<char>::from_iter(rucksack.compartment_two.chars());

    compartment_one.intersection(&compartment_two).collect::<Vec<&char>>()[0].to_string()
}

fn find_common_item_type_among_rucksacks(rucksacks: Vec<String>) -> String {
    if rucksacks.len() > 2 {
        let first_sack = HashSet::<char>::from_iter(rucksacks[0].chars());
        let second_sack = HashSet::<char>::from_iter(rucksacks[1].chars());
        let mut result = vec![
            first_sack.intersection(&second_sack).collect::<String>()
        ];
        rucksacks.into_iter().skip(2).for_each(|sack| result.push(sack));
        return find_common_item_type_among_rucksacks(result)
    }

    let first_sack = HashSet::<char>::from_iter(rucksacks[0].chars());
    let second_sack = HashSet::<char>::from_iter(rucksacks[1].chars());

    first_sack.intersection(&second_sack).collect::<Vec<&char>>()[0].to_string()
}

fn priority_of_item_type(item_type: &str) -> i32 {
    let item_type_priority = HashMap::from([ 
        ("a", 1),
        ("b", 2),
        ("c", 3),
        ("d", 4),
        ("e", 5),
        ("f", 6),
        ("g", 7),
        ("h", 8),
        ("i", 9),
        ("j", 10),
        ("k", 11),
        ("l", 12),
        ("m", 13),
        ("n", 14),
        ("o", 15),
        ("p", 16),
        ("q", 17),
        ("r", 18),
        ("s", 19),
        ("t", 20),
        ("u", 21),
        ("v", 22),
        ("w", 23),
        ("x", 24),
        ("y", 25),
        ("z", 26),
        ("A", 27),
        ("B", 28),
        ("C", 29),
        ("D", 30),
        ("E", 31),
        ("F", 32),
        ("G", 33),
        ("H", 34),
        ("I", 35),
        ("J", 36),
        ("K", 37),
        ("L", 38),
        ("M", 39),
        ("N", 40),
        ("O", 41),
        ("P", 42),
        ("Q", 43),
        ("R", 44),
        ("S", 45),
        ("T", 46),
        ("U", 47),
        ("V", 48),
        ("W", 49),
        ("X", 50),
        ("Y", 51),
        ("Z", 52),
    ]);

    item_type_priority[item_type]
}

struct Rucksack {
    compartment_one: String,
    compartment_two: String,
}
