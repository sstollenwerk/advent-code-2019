from copy import deepcopy

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
    flatten,
)


def make_input(n: int) -> list[int]:
    for x in range(n):
        for y in range(n):
            yield (x, y)


def part1(s: str) -> int:
    code = parse_intcode(s)
    input_ = make_input(50)
    data = {}

    for i in input_:

        f = iter(i).__next__
        res = next(iterpret_intcode(deepcopy(code), f))
        data[i] = res

    print(as_display(data))

    return sum(data.values())


def part2(s: str) -> int:
    pass


def main():
    s = get_file(19)
    print(f"{part1(s)=}")
    print(f"{part2(s)=}")


if __name__ == "__main__":
    main()
