from dataclasses import dataclass
from utils import chunk_list

@dataclass
class Rucksack:
    compartment_one: str
    compartment_two: str

def part1() -> None:
    common_items = list(map(find_common_item_type_in_rucksack, lines_to_rucksacks()))
    common_items_with_priority = list(map(priority_of_item_type, common_items))

    print(f'Day 3 Part 1 solution: {sum(common_items_with_priority)}')

def part2() -> None:
    emptied_racksacks = list(map(lambda sack: sack.compartment_one + sack.compartment_two, lines_to_rucksacks()))
    emptied_racksacks_grouped = chunk_list(emptied_racksacks, chunk_size=3)
    common_items = list(map(find_common_item_type_among_rucksacks, emptied_racksacks_grouped))
    common_items_with_priority = list(map(priority_of_item_type, common_items))

    print(f'Day 3 Part 2 solution: {sum(common_items_with_priority)}')

def lines_to_rucksacks() -> list[Rucksack]:
    rucksacks: list[Rucksack] = []

    with open('src/day3/data.txt') as f:
        for line in f:
            rucksacks.append(
                Rucksack(
                    compartment_one=line[:int(len(line) / 2)].strip(),
                    compartment_two=line[int(len(line) / 2):].strip(),
                )
            )

    return rucksacks

def find_common_item_type_in_rucksack(rucksack: Rucksack) -> str:
    return  ''.join(set(rucksack.compartment_one) & set(rucksack.compartment_two))

def find_common_item_type_among_rucksacks(rucksacks: list[str]) -> str:
    if len(rucksacks) > 2:
        result = [
            ''.join(set(rucksacks[0]) & set(rucksacks[1]))
        ]
        result = result + rucksacks[2:]
        return find_common_item_type_among_rucksacks(result)

    return list(set(rucksacks[0]) & set(rucksacks[1]))[0]

def priority_of_item_type(item_type: str) -> int:
    if ITEM_TYPE_PRIORITY[item_type] is not None:
        return ITEM_TYPE_PRIORITY[item_type]
    else:
        raise Exception(f'Invalid item type. Valid item types and prioritys {ITEM_TYPE_PRIORITY}')


ITEM_TYPE_PRIORITY = {
    'a': 1,
    'b': 2,
    'c': 3,
    'd': 4,
    'e': 5,
    'f': 6,
    'g': 7,
    'h': 8,
    'i': 9,
    'j': 10,
    'k': 11,
    'l': 12,
    'm': 13,
    'n': 14,
    'o': 15,
    'p': 16,
    'q': 17,
    'r': 18,
    's': 19,
    't': 20,
    'u': 21,
    'v': 22,
    'w': 23,
    'x': 24,
    'y': 25,
    'z': 26,
    'A': 27,
    'B': 28,
    'C': 29,
    'D': 30,
    'E': 31,
    'F': 32,
    'G': 33,
    'H': 34,
    'I': 35,
    'J': 36,
    'K': 37,
    'L': 38,
    'M': 39,
    'N': 40,
    'O': 41,
    'P': 42,
    'Q': 43,
    'R': 44,
    'S': 45,
    'T': 46,
    'U': 47,
    'V': 48,
    'W': 49,
    'X': 50,
    'Y': 51,
    'Z': 52,
}
