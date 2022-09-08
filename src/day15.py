from functools import partial
from copy import deepcopy
from collections import defaultdict
from itertools import islice
from time import sleep
from ast import literal_eval

from more_itertools import grouper
import keyboard


from generic import (
    get_file,
    parse_intcode,
    iterpret_intcode,
    cat_maybes,
    IntCode,
    Position,
    as_pos,
    map_vals,
    map_keys,
    as_display,
    rotate_counterclockwise,
    as_grid,
    fill,
    graph_search,
)


def cmp(a, b) -> int:
    # from https://stackoverflow.com/questions/22490366/how-to-use-cmp-in-python-3
    return (a > b) - (a < b)


def display(grid: dict[Position | complex, int]) -> None:
    disp = {-1: " ", 0: "#", 1: ".", 2: "*", 3: "D"}.__getitem__
    data = map_vals(disp, fill(grid, -1))
    data = map_keys(as_pos, data)
    data = map_keys(rotate_counterclockwise, data)

    show_ = as_grid(data)

    s = "\n".join("".join(r) for r in show_)
    print(s)


def get_neighbours(vals: set[complex]):
    def inner(p: complex) -> list[complex]:
        return vals & {p + d for d in {1, -1, 1j, -1j}}

    return inner


def part1(s: str) -> int:
    ##code = parse_intcode(s)
    ##grid =  move(code)
    grid = literal_eval(get_file("hardcoded_day15_grid"))
    display(grid)
    # solved maze before doing dijkastra stuff.
    start = 0 + 0j
    end = next((k for k, v in grid.items() if v == 2))
    floor = {k for k, v in grid.items() if v >= 1}
    is_end = lambda x: x == end
    neighbours = get_neighbours(floor)
    print(neighbours(16j))

    return graph_search(start, neighbours, is_end)


def get_input() -> int:
    posses = {"w": 1, "s": 2, "d": 3, "a": 4}
    print("move: ")
    while (c := keyboard.read_key(suppress=True)) not in posses.keys():
        if c == "esc":
            raise EOFError
        continue
    sleep(0.1)
    p = posses[c]
    global MOVED
    MOVED = p
    return p


MOVED: int


def move(code):

    ask = get_input

    comp = iterpret_intcode(code, ask)

    pos = 0 + 0j
    # 3 = droid
    # 2 = oxygen
    grid = {pos: 1}

    deltas = {1: 1j, 2: -1j, 3: 1, 4: -1}
    try:
        for c in comp:
            new_pos = pos + deltas[MOVED]
            grid[new_pos] = c
            print(c)
            if c:
                pos = new_pos
            tmp = grid.copy()
            tmp[pos] = 3
            display(tmp)
    except EOFError:  # ctrl+z
        return grid


def part2(s: str) -> int:
    pass


def main():
    s = get_file(15)
    print(f"{part1(s)=}")
    print(f"{part2(s)=}")


if __name__ == "__main__":
    main()
