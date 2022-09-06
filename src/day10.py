from collections import defaultdict
from math import atan2, tau
import copy

from typing import Callable, TypeVar
from collections.abc import Iterable


from generic import get_file, position, bin_items, ilen, to_d, as_grid, delta, tup_add


asteriod = position


def parse_asteroids(s: str) -> frozenset[asteriod]:
    return frozenset(
        ((y, x) for (y, r) in enumerate(s.split()) for x, c in enumerate(r) if c == "#")
    )


def find_angle(point: position) -> float:
    """
    angle between vertical line and (origin - point)
    worried this might cause issues with floating point error
    """
    from_horiz = atan2(*point)
    vert = atan2(-1, 0)
    place = from_horiz - vert
    place %= tau
    return place


def asteroid_groups(
    asters: frozenset[asteriod],
) -> Callable[[asteriod], dict[float, list[asteriod]]]:
    def inner(a: asteriod) -> dict[float, list[asteriod]]:
        places = {delta(b, a) for b in asters} - {(0, 0)}
        return dict(bin_items(find_angle, places))

    return inner


def best_seen(asters: frozenset[asteriod]) -> (int, asteriod):
    f = asteroid_groups(asters)
    posses = ((ilen(f(a).keys()), a) for a in asters)
    return max(posses)


def most_seen(asters: frozenset[asteriod]) -> int:
    return best_seen(asters)[0]


def destroy_asteroids(places_: dict[float, list[asteriod]]) -> Iterable[asteriod]:
    places = copy.deepcopy(places_)
    for k, v in places.items():
        places[k] = sorted(v, key=lambda x: abs(complex(*x)), reverse=True)

    while places:
        for k in sorted(places.keys()):
            v = places[k]
            yield v.pop()
            if not v:
                del places[k]


def part1(s: str) -> int:
    return most_seen(parse_asteroids(s))


def part2(s: str) -> int:
    asters = parse_asteroids(s)
    ast = best_seen(asters)[1]
    places = asteroid_groups(asters)(ast)

    r = [tup_add(ast, i) for i in list(destroy_asteroids(places))][199]
    return r[1] * 100 + r[0]


def main():
    s = get_file(10)
    print(f"{part1(s)=}")
    print(f"{part2(s)=}")


if __name__ == "__main__":
    main()
