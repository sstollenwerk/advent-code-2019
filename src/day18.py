from itertools import cycle, accumulate
from operator import mul
from collections.abc import Iterable
from functools import cache

from dataclasses import dataclass

from more_itertools import nth

from generic import (
    get_file,
    iterate,
    map_keys,
    T,
    V,
    flatten,
    graph_search,
    weighted_graph_search,
    weighted_acyclic_digraph_search,
    dfs_weighted_graph_search,
    astar,
)


frozDict = frozenset[tuple[T, V]]


def to_frozdict(d: dict[T, V]) -> frozDict:
    return frozenset(d.items())


def from_frozdict(d: frozDict) -> dict[T, V]:
    return dict(d)


@dataclass
class Maze:
    location: complex
    floor: frozenset[complex]
    keys: dict[str, complex]
    doors: dict[str, complex]

    @classmethod
    def from_str(cls, n: str):
        nodes = {
            complex(i, j): c
            for i, row in enumerate(n.split())
            for j, c in enumerate(row)
            if c != "#"
        }
        start = next(k for k, v in nodes.items() if v == "@")

        floor = frozenset({k for k, v in nodes.items()})
        keys = {v: k for k, v in nodes.items() if v.islower()}
        doors = {v: k for k, v in nodes.items() if v.isupper()}

        return cls(start, floor, keys, doors)


Node = [frozenset[str], complex]
Edge = tuple[Node, int]


def adjacences(c: complex) -> set[complex]:
    return {c + d for d in {1, -1, 1j, -1j}}


def get_neighbours_alt(m: Maze):
    nodes = m.floor

    def neighbours(n: Node) -> Edge:
        seen, start = n
        to_see = set(m.keys.keys()) - seen

        to_open = {m.doors.get(i.upper()) for i in to_see}

        dests = {v: k for k, v in m.keys.items() if k in to_see}

        nei = lambda c: (adjacences(c) & nodes) - to_open

        step = {start}
        res = {}
        seen_ = set()
        i = 0
        while step:
            for k in step:
                res[k] = i
            next_ = set(flatten(map(nei, step))) - seen_
            seen_ |= step
            i += 1
            step = next_

        return sorted([((seen | {v}, k), res[k]) for k, v in dests.items() if k in res])

    return neighbours


def get_neighbours(m: Maze):
    nodes = m.floor

    # @cache
    def neighbours(n: Node) -> Edge:
        seen, start = n
        to_see = set(m.keys.keys()) - seen
        to_open = {m.doors.get(i.upper()) for i in to_see}

        nei = lambda c: [(i, 1) for i in (adjacences(c) & nodes) - to_open]

        res = []
        for key in to_see:
            pos = m.keys[key]
            heur = lambda x: manhatten(x, pos)
            success = lambda x: heur(x) == 0
            dist = astar(start, nei, heur, success)
            if dist is None:
                continue
            res.append(((seen | {key}, pos), dist))

        return sorted(res)

    return neighbours


def manhatten(a, b):
    k = b - a
    return abs(k.real) + abs(k.imag)


def part1(s: str) -> int:
    maze = Maze.from_str(s)

    success = lambda m: m[0] == frozenset(maze.keys.keys())

    print(maze)

    neighbours = get_neighbours_alt(maze)

    start = (frozenset(), maze.location)

    r = weighted_graph_search(start, neighbours, success)

    print(r)

    return min(r.values())


def part2(n: Node) -> int:
    pass


def main():
    s = """#################
#i.G..c...e..H.p#
########.########
#j.A..b...f..D.o#
########@########
#k.E..a...g..B.n#
########.########
#l.F..d...h..C.m#
#################"""
    s = get_file(18).strip()

    print(f"{part1(s)=}")
    print(f"{part2(s)=}")


if __name__ == "__main__":
    main()
