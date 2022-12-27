def part_1() -> None:
    number_overlapping: int = 0

    for (first, second) in lines_to_assignments():
        first_contains_second = first[0] <= second[0] and first[-1] >= second[-1]
        second_contains_first = second[0] <= first[0] and second[-1] >= first[-1]

        if (first_contains_second or second_contains_first):
            number_overlapping += 1

    print(f'Day 4 Part 1 solution: {number_overlapping}')

def part_2() -> None:
    number_overlapping: int = 0

    for (first, second) in lines_to_assignments():
        first_overlaps_second = first[0] >= second[0] and first[0] <= second[-1]
        second_overlaps_first = second[0] >= first[0] and second[0] <= first[-1]

        if (first_overlaps_second or second_overlaps_first):
            number_overlapping += 1

    print(f'Day 4 Part 2 solution: {number_overlapping}')

def lines_to_assignments() -> list[tuple[list[int], list[int]]]:
    assignments: list[tuple[list[int], list[int]]] = []

    with open('src/day4/data.txt') as f:
        for line in f:
            first, second = line.split(',')
            assignments.append(
                (parse_assignment(first), parse_assignment(second))
            )

    return assignments

def parse_assignment(assignment_data: str) -> list[int]:
    first, second = assignment_data.split('-')
    return list(range(int(first), int(second) + 1))
