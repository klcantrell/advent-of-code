from dataclasses import dataclass
from functools import reduce
from typing import Callable

@dataclass
class Score:
    outcome_points: int
    shape_points: int

def part1() -> None:
    total_score = lines_to_scores(outcome_points_of_round_part1, points_for_shape_part1)
    result = reduce(lambda total, round: total + round.outcome_points + round.shape_points, total_score, 0)
    print(f'Day 2 Part 1 solution: {result}')

def part2() -> None:
    total_score = lines_to_scores(outcome_points_of_round_part2, points_for_shape_part2)
    result = reduce(lambda total, round: total + round.outcome_points + round.shape_points, total_score, 0)
    print(f'Day 2 Part 2 solution: {result}')

def outcome_points_of_round_part1(other: str, selection: str) -> int:
    match (other, selection):
        case ('A', 'X'): return 3
        case ('A', 'Y'): return 6
        case ('A', 'Z'): return 0
        case ('B', 'X'): return 0
        case ('B', 'Y'): return 3
        case ('B', 'Z'): return 6
        case ('C', 'X'): return 6
        case ('C', 'Y'): return 0
        case ('C', 'Z'): return 3
        case _: raise Exception('Other must be in one \"A\", \"B\", or \"C\". Selection must be in one \"X\", \"Y\", or \"Z\"')

def points_for_shape_part1(other: str, selection: str) -> int:
    match selection:
        case 'X': return 1
        case 'Y': return 2
        case 'Z': return 3
        case _: raise Exception('Selection must be in one \"X\", \"Y\", or \"Z\"')

def outcome_points_of_round_part2(other: str, selection: str) -> int:
    match selection:
        case 'X': return 0
        case 'Y': return 3
        case 'Z': return 6
        case _: raise Exception('Selection must be in one \"X\", \"Y\", or \"Z\"')

def points_for_shape_part2(other: str, selection: str) -> int:
    match (other, selection):
        case ('A', 'X'): return 3
        case ('A', 'Y'): return 1
        case ('A', 'Z'): return 2
        case ('B', 'X'): return 1
        case ('B', 'Y'): return 2
        case ('B', 'Z'): return 3
        case ('C', 'X'): return 2
        case ('C', 'Y'): return 3
        case ('C', 'Z'): return 1
        case _: raise Exception('Other must be in one \"A\", \"B\", or \"C\". Selection must be in one \"X\", \"Y\", or \"Z\"')

def lines_to_scores(
    outcome_points_calculator: Callable[[str, str], int],
    shape_points_calculator: Callable[[str, str], int],
) -> list[Score]:
    scores: list[Score] = []

    with open('src/day2/data.txt') as f:
        for line in f:
            round_input = list(map(str.strip, line.split(' ')))
            if len(round_input) != 2:
                raise Exception('A round should be 3 characters with the second being a space (e.g. A X)')
            scores.append(
                Score(
                    outcome_points=outcome_points_calculator(round_input[0], round_input[1]),
                    shape_points=shape_points_calculator(round_input[0], round_input[1])
                )
            )
    
    return scores
