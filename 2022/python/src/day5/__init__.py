from enum import Enum
from dataclasses import dataclass
import re

from utils import chunk_list

@dataclass 
class MoveInstruction:
    source_stack: int
    destination_stack: int
    amount_to_move: int

# @dataclass generates a class with an __init__ like below
# see https://docs.python.org/3/library/dataclasses.html for what else it does

# class MoveInstruction:

#     def __init__(self, source_stack: str, destination_stack: str, amount_to_move: int) -> None:
#         self.source_stack = source_stack
#         self.destination_stack = destination_stack
#         self.amount_to_move = amount_to_move

class ParseState(Enum):
    PARSING_INITIAL_STACK = 0
    PARSING_INSTRUCTIONS = 1

STACK_ITEM_SIZE = 4

parse_state: ParseState = ParseState.PARSING_INITIAL_STACK
stacks: list[list[str]] = []
instructions: list[MoveInstruction] = []

def part_1() -> None:
    parse_lines()

    for instruction in instructions:
        negative_index_of_items_to_move = -1 * instruction.amount_to_move
        source_stack_index = instruction.source_stack - 1
        destination_stack_index = instruction.destination_stack - 1

        source_stack = stacks[source_stack_index]
        items_to_move: list[str] = source_stack[negative_index_of_items_to_move:]
        items_to_move.reverse()

        stacks[destination_stack_index].extend(items_to_move)
        stacks[source_stack_index] = source_stack[:negative_index_of_items_to_move]

    stack_items_on_top = [x[-1] for x in stacks if x[-1] != '']

    print(f'Day 5 Part 1 solution: {"".join(stack_items_on_top)}')

def part_2() -> None:
    parse_lines()

    for instruction in instructions:
        negative_index_of_items_to_move = -1 * instruction.amount_to_move
        source_stack_index = instruction.source_stack - 1
        destination_stack_index = instruction.destination_stack - 1

        source_stack = stacks[source_stack_index]
        items_to_move: list[str] = source_stack[negative_index_of_items_to_move:]

        stacks[destination_stack_index].extend(items_to_move)
        stacks[source_stack_index] = source_stack[:negative_index_of_items_to_move]

    stack_items_on_top = [x[-1] for x in stacks if x[-1] != '']

    print(f'Day 5 Part 2 solution: {"".join(stack_items_on_top)}')

def parse_lines() -> None:
    reset_parse_state()

    global parse_state
    global stacks

    with open('src/day5/data.txt') as f:
        for line in f:
            line = line.replace('\n', '')
            match parse_state:
                case ParseState.PARSING_INITIAL_STACK:
                    if is_stack_bottom(line):
                        parse_state = ParseState.PARSING_INSTRUCTIONS
                        continue
                    else:
                        for i, stack_item in enumerate(parse_stack_row(line)):
                            if i > len(stacks) - 1:
                                stacks.append([stack_item])
                            else:
                                stacks[i].insert(0, stack_item)

                case ParseState.PARSING_INSTRUCTIONS:
                    if is_valid_instruction(line):
                        instructions.append(parse_instruction(line))
                    else:
                        continue

    stacks = [[stack_item for stack_item in stack if stack_item != ''] for stack in stacks]

def is_stack_bottom(raw_row: str) -> bool:
    try:
        return int(raw_row[1]) is not None
    except:
        return False

def parse_stack_row(raw_row: str) -> list[str]:
    stack_items = chunk_list(list(raw_row), chunk_size = STACK_ITEM_SIZE)
    return [stack_item[1] if stack_item[1] != ' ' else '' for stack_item in stack_items]

def is_valid_instruction(raw_instruction: str) -> bool:
    return raw_instruction.startswith('move')

def parse_instruction(raw_instruction: str) -> MoveInstruction:
    instruction_values_raw = [x for x in re.split(r'move | from | to ', raw_instruction) if x != '']
    return MoveInstruction(
        amount_to_move=int(instruction_values_raw[0]),
        source_stack=int(instruction_values_raw[1]),
        destination_stack=int(instruction_values_raw[2]),
    )

def reset_parse_state() -> None:
    global parse_state
    global stacks
    global instructions

    parse_state = ParseState.PARSING_INITIAL_STACK
    stacks = []
    instructions = []
