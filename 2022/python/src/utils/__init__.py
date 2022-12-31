from typing import TypeVar

T = TypeVar('T')

def chunk_list(original_list: list[T], chunk_size: int) -> list[list[T]]:
    return [original_list[i:i + chunk_size] for i in range(0, len(original_list), chunk_size)]
