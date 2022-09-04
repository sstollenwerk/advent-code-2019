from itertools import groupby
from math import prod

from collections.abc import Iterable

from generic import ilen, get_file

from collections import defaultdict

position = tuple[int, int]

def make_stack(size:tuple[int, int],  s:str) -> defaultdict[position, list[int]]:
    r = defaultdict(list)
    for j,c in enumerate(map(int, s)):
        i = j % prod(size)
        p = divmod(i, size[0])
        r[p].append(c)
    return r

def get_score(layer:list[int]) -> int:
    return layer.count(1) * layer.count(2)

def part1(s: str) -> int:
    data = make_stack( (25, 6), s)
    layers = zip(*data.values())
    best = max(layers, key = lambda x: sum(map(bool, x)))
    return get_score(best)

def part2(s: str) -> int:
    pass


def main():
    s = get_file(8).strip()
    print(f"{part1(s)=}")
    print(f"{part2(s)=}")


if __name__ == "__main__":
    main()
