use crate::utils;

pub fn part_1() {
    let total_score = lines_to_scores(outcome_points_of_round_part1, points_for_shape_part1);
    println!(
        "Day 2 Part 1 solution: {}",
        total_score.iter().fold(0, |total, round| total + round.outcome_points + round.shape_point)
    );
}

pub fn part_2() {
    let total_score = lines_to_scores(outcome_points_of_round_part2, points_for_shape_part2);
    println!(
        "Day 2 Part 2 solution: {}",
        total_score.iter().fold(0, |total, round| total + round.outcome_points + round.shape_point)
    );
}

fn lines_to_scores<T, U>(
    outcome_points_calculator: T,
    shape_points_calculator: U
) -> Vec<Score> where T: Fn(&str, &str) -> i32, U: Fn(&str, &str) -> i32 {
    let mut scores: Vec<Score> = vec![];

    if let Ok(lines) = utils::read_lines("./data/day2.txt") {
        for line in lines.flatten() {
            let round_input = line.split(' ').collect::<Vec<&str>>();
            if round_input.len() != 2 {
                panic!("A round should be 3 characters with the second being a space (e.g. A X)")
            }
            scores.push(Score {
                outcome_points: outcome_points_calculator(round_input[0], round_input[1]),
                shape_point: shape_points_calculator(round_input[0], round_input[1]),
            })
        }
    }

    scores
}

fn outcome_points_of_round_part1(other: &str, selection: &str) -> i32 {
    match (other, selection) {
        ("A", "X") => 3,
        ("A", "Y") => 6,
        ("A", "Z") => 0,
        ("B", "X") => 0,
        ("B", "Y") => 3,
        ("B", "Z") => 6,
        ("C", "X") => 6,
        ("C", "Y") => 0,
        ("C", "Z") => 3,
        _ => panic!("Other must be in one \"A\", \"B\", or \"C\". Selection must be in one \"X\", \"Y\", or \"Z\"")
    }
}

fn outcome_points_of_round_part2(_other: &str, selection: &str) -> i32 {
    match selection {
        "X" => 0,
        "Y" => 3,
        "Z" => 6,
        _ => panic!("Selection must be in one \"X\", \"Y\", or \"Z\"")
    }
}

fn points_for_shape_part1(_other: &str, selection: &str) -> i32 {
    match selection {
        "X" => 1,
        "Y" => 2,
        "Z" => 3,
        _ => panic!("Selection must be in one \"X\", \"Y\", or \"Z\"")
    }
}

fn points_for_shape_part2(other: &str, selection: &str) -> i32 {
    match (other, selection) {
        ("A", "X") => 3,
        ("A", "Y") => 1,
        ("A", "Z") => 2,
        ("B", "X") => 1,
        ("B", "Y") => 2,
        ("B", "Z") => 3,
        ("C", "X") => 2,
        ("C", "Y") => 3,
        ("C", "Z") => 1,
        _ => panic!("Other must be in one \"A\", \"B\", or \"C\". Selection must be in one \"X\", \"Y\", or \"Z\"")
    }
}

struct Score {
    outcome_points: i32,
    shape_point: i32,
}
