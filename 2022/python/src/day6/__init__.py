PACKET_LENGTH = 4
MESSAGE_LENGTH = 14

def part_1() -> None:
    window_start = 0
    datastream = get_input()
    current_window = datastream[window_start:(window_start + PACKET_LENGTH)]
    while not is_start_of_packet(current_window):
        window_start += 1
        current_window = datastream[window_start:(window_start + PACKET_LENGTH)]
    print(f'Day 6 part 1 solution: {window_start + PACKET_LENGTH}')

def part_2() -> None:
    window_start = 0
    datastream = get_input()
    current_window = datastream[window_start:(window_start + MESSAGE_LENGTH)]
    while not is_start_of_message(current_window):
        window_start += 1
        current_window = datastream[window_start:(window_start + MESSAGE_LENGTH)]
    print(f'Day 6 part 2 solution: {window_start + MESSAGE_LENGTH}')

def is_start_of_packet(window: str) -> bool:
    return len(set(window)) == PACKET_LENGTH

def is_start_of_message(window: str) -> bool:
    return len(set(window)) == MESSAGE_LENGTH

def get_input() -> str:
    with open('src/day6/data.txt') as f:
        return f.readline().strip()
