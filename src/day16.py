from itertools import cycle
from operator import mul
from collections.abc import Iterable

from more_itertools import nth

from generic import (
    get_file,
    iterate,
    map_vals,
    T,
    flatten,
)


Signal = list[int]


def parse(s: str) -> Signal:
    return list(map(int, s))


def part(sig: Signal, n: int) -> int:
    pat = make_pattern(n)
    r = sum(map(mul, sig, pat))
    return abs(r) % 10


def make_pattern(n: int) -> Iterable[int]:
    base = [0, 1, 0, -1]
    r = cycle(flatten([i] * n for i in base))
    next(r)
    return r


def step(s: Signal) -> Signal:
    return [part(s, i + 1) for i in range(len(s))]


def part1(s):
    signal = parse(s)
    res = nth(iterate(step, signal), 100)
    return int("".join(map(str, res))[:8])


def part2(s):
    pass


def main():
    s = get_file(16).strip()
    print(f"{part1(s)=}")
    print(f"{part2(s)=}")


if __name__ == "__main__":
    main()
