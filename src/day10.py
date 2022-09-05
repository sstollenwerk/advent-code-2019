from collections import defaultdict
from math import atan2

from typing import Callable, TypeVar

from generic import get_file, position, bin_items, ilen, to_d, as_grid, delta


asteriod = position


def parse_asteroids(s: str) -> frozenset[asteriod]:
    return frozenset(
        ((y, x) for (y, r) in enumerate(s.split()) for x, c in enumerate(r) if c == "#")
    )


def find_angle(point: position) -> float:
    """
    angle between horizontal line and (origin - point)
    """
    p = atan2(*point)
    return p


def asteroid_groups(
    asters: frozenset[asteriod],
) -> Callable[[asteriod], dict[float, list[asteriod]]]:
    def inner(a: asteriod) -> dict[float, list[asteriod]]:
        places = {delta(b, a) for b in asters} - {(0, 0)}
        return dict(bin_items(find_angle, places))

    return inner


def most_seen(asters: frozenset[asteriod]) -> int:
    f = asteroid_groups(asters)
    return max(map(ilen, map(dict.keys, map(f, asters))))


def part1(s: str) -> int:
    return most_seen(parse_asteroids(s))


def part2(s: str) -> int:
    pass


def main():
    poss = """.#..#
.....
#####
....#
...##"""
    list(map(print, as_grid(to_d(parse_asteroids(poss)))))
    # verify that parse, display works.

    s = get_file(10)
    print(f"{part1(s)=}")
    print(f"{part2(s)=}")


if __name__ == "__main__":
    main()
