from itertools import count

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
    tup_add,
)


def make_input(n: int) -> list[int]:
    for x in range(n):
        for y in range(n):
            yield (x, y)


def find_gridsize(s, data):
    for i in count(start=0):
        dels = [complex(real=i, imag=0), complex(real=0, imag=-i)]
        dels.append(dels[0] + dels[1])
        for d in dels:
            p = s + d
            if not data.get(p, False):
                return i


def part1(s: str) -> int:
    code = parse_intcode(s)
    data = {}

    start = 0

    for y in range(50):
        seen = None
        for x in range(start, 50):
            i_ = (x, y)
            f = iter(i_).__next__
            i = complex(real=x, imag=y)
            res = bool(next(iterpret_intcode(code, f)))
            data[i] = res
            if not (seen) and res:
                seen = i
                start = x
            if seen is not None and not res:
                break
        # print(as_display(data))

    print(as_display(data))

    return sum(data.values())


def part2(s: str) -> int:

    code = parse_intcode(s)
    data = {}

    start = 0

    for y in count(0):
        seen = None
        for x in count(start):
            i_ = (x, y)
            f = iter(i_).__next__
            i = complex(real=x, imag=y)
            res = bool(next(iterpret_intcode(code, f)))
            # if res:
            #    print(i)
            data[i] = res
            if x - 10 > start and seen is None:
                break
            if not (seen) and res:
                seen = i
                start = x
            if seen is not None and not res:
                break
        if seen:
            m = 100
            if find_gridsize(seen, data) == m:
                x, y_ = as_pos(seen)
                y = y_ - (m - 1)
                return x * 10000 + y

        # print(as_display(data))


def main():
    s = get_file(19)
    print(f"{part1(s)=}")

    print(f"{part2(s)=}")


if __name__ == "__main__":
    main()
