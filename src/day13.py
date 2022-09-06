from functools import partial
from copy import deepcopy
from collections import defaultdict

from more_itertools import grouper

from generic import (
    get_file,
    parse_intcode,
    iterpret_intcode,
    cat_maybes,
    IntCode,
    Position,
    as_pos,
    map_keys,
    as_display,
    rotate_counterclockwise,
    as_grid,
)


def start_game(code: IntCode) -> dict[Position, bool]:
    comp = iterpret_intcode(code)
    data =  grouper(cat_maybes(comp), 3, incomplete='ignore') 
    grid = { (b,a): c for (a,b,c) in data}
    
    return grid


def part1(s: str) -> int:
    code = parse_intcode(s)
    return list(start_game(code).values()).count(2)


def part2(s: str) -> None:
    pass


def main():
    s = get_file(13)
    print(f"{part1(s)=}")
    print(f"{part2(s)=}")


if __name__ == "__main__":
    main()
