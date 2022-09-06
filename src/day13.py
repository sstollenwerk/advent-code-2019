from functools import partial
from copy import deepcopy
from collections import defaultdict
from itertools import islice

from more_itertools import grouper

from generic import (
    get_file,
    parse_intcode,
    iterpret_intcode,
    cat_maybes,
    IntCode,
    Position,
    as_pos,
    map_vals,
    as_display,
    rotate_counterclockwise,
    as_grid,
)


def cmp(a, b) -> int:
    # from https://stackoverflow.com/questions/22490366/how-to-use-cmp-in-python-3
    return (a > b) - (a < b)


def display(grid: dict[Position, int]) -> None:
    disp = [" ", "x", "b", "@", "o"].__getitem__
    data = map_vals(disp, grid)

    show_ = as_grid(data)

    s = "\n".join("".join(r) for r in show_)
    print(s)


def start_game(code: IntCode) -> dict[Position, int]:
    comp = iterpret_intcode(code)
    data = grouper(cat_maybes(comp), 3, incomplete="ignore")
    grid = {(b, a): c for (a, b, c) in data}

    return grid


def part1(s: str) -> int:
    code = parse_intcode(s)
    return list(start_game(code).values()).count(2)


def get_input() -> int:
    adj = {"z": -1, "x": 0, "c": 1}
    adj |= {str(v): v for v in adj.values()}
    while (c := input("move")) not in adj.keys():
        continue
    return adj[c]


def play(code):

    ##ask = get_input

    # easier to get the computer to play

    get_ = lambda g, check: [k for k, v in g.items() if v == check][0]
    ask = lambda: cmp(get_(grid, 4)[1], get_(grid, 3)[1])

    comp = iterpret_intcode(code, ask)
    data = grouper(islice(comp, 0, (1 + 25) * (1 + 44) * 3), 3, incomplete="ignore")
    # hardcoded size of field
    grid = {(b, a): c for (a, b, c) in data}

    ##display(grid)

    for (a, b, c) in grouper(comp, 3, incomplete="ignore"):
        if (a, b) == (-1, 0):
            yield (c)
        else:
            grid[(b, a)] = c
        ##display(grid)


def part2(s: str) -> int:
    code = parse_intcode(s)
    code[0] = 2

    return list(play(code))[-1]


def main():
    s = get_file(13)
    print(f"{part1(s)=}")
    print(f"{part2(s)=}")


if __name__ == "__main__":
    main()
