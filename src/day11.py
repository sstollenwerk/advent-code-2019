from functools import partial
from copy import deepcopy
from collections import defaultdict

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
)


def make_panel(code: IntCode, start=0) -> dict[Position, bool]:
    panel = defaultdict(bool)
    position = 0 + 0j
    panel[position] = bool(start)
    direction = 0 + 1j
    rotation = [0 + 1j, 0 - 1j].__getitem__
    comp = iterpret_intcode(code)
    next(comp)
    while True:
        try:
            paint = comp.send(int(panel[position]))
            rotate = next(comp)
            next(comp)
        except StopIteration:
            return map_keys(as_pos, panel)
        else:
            panel[position] = bool(paint)
            direction *= rotation(rotate)
            position += direction


def part1(s: str) -> int:
    code = parse_intcode(s)
    return len(make_panel(code).keys())


def part2(s: str) -> None:
    code = parse_intcode(s)
    print(as_display(map_keys(rotate_counterclockwise, make_panel(code, 1))))


def main():
    s = get_file(11)
    print(f"{part1(s)=}")
    print(f"{part2(s)=}")


if __name__ == "__main__":
    main()
