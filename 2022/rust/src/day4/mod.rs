use crate::utils;

pub fn part_1() {
    let mut count = 0;

    for (first, second) in lines_to_assignments().iter() {
        let first_contains_second = first.start <= second.start && first.end >= second.end;
        let second_contains_first = second.start <= first.start && second.end >= first.end;

        if first_contains_second || second_contains_first {
            count += 1;
        }
    }

    println!("Day 4 Part 1 solution: {}", count);
}

fn lines_to_assignments() -> Vec<(Assignment, Assignment)> {
    let mut assignments: Vec<(Assignment, Assignment)> = vec![];

    if let Ok(lines) = utils::read_lines("./data/day4.txt") {
        for line in lines.flatten() {
            let line = line.split(',').collect::<Vec<&str>>();
            if line.len() != 2 {
                panic!(
                    "Each line should contain two assignments separated by a comma (e.g. 1-4,2-3)"
                )
            }
            assignments.push((parse_assignment(line[0]), parse_assignment(line[1])))
        }
    }

    assignments
}

fn parse_assignment(assignment_data: &str) -> Assignment {
    let assignment_data = assignment_data.split('-').collect::<Vec<&str>>();
    if assignment_data.len() != 2 {
        panic!("An assignment should be two numbers separated by a dash (e.g. 1-4)")
    }
    Assignment {
        start: assignment_data[0].parse::<i8>().unwrap(),
        end: assignment_data[1].parse::<i8>().unwrap(),
    }
}

#[derive(Debug)]
struct Assignment {
    start: i8,
    end: i8,
}
