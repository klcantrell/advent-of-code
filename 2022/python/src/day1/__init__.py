def part1() -> None:
    print(f'Day 1 Part 1 solution: {max(calories_by_elf())}')

def part2() -> None:
    calories_by_elf_sorted_descending = sorted(calories_by_elf(), reverse=True)
    print(f'Day 1 Part 2 solution: {sum(calories_by_elf_sorted_descending[:3])}')

def calories_by_elf() -> list[int]:
    parsed_data: list[list[int]] = []
    group: list[int] = []

    with open('src/day1/data.txt') as f:
        for line in f:
            if line == "\n":
                parsed_data.append(group)
                group = []
            else:
                group.append(int(line))

        if len(group) > 0:
            parsed_data.append(group)

    return list(map(lambda x: sum(x), parsed_data))
