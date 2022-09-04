from dataclasses import dataclass
from collections import defaultdict
from functools import cache

from typing import Callable, TypeVar
from collections.abc import Iterable


from generic import get_file


Node = TypeVar("Node")


Parents = dict[Node, Node]


def path_finder(parents: Parents) -> Callable[[Node], list[Node]]:
    @cache
    def find_path(n: Node) -> list[Node]:
        p = parents.get(n)
        if not p:
            return []
        return [p] + find_path(p)

    return find_path


def make_parents(s: str) -> Parents:
    return dict(r.split(")")[::-1] for r in s.split())


def all_nodes(p: Parents) -> set[Node]:
    return set(p.keys()) | set(p.values())


def part1(s: str) -> int:
    par = make_parents(s)
    f = path_finder(par)
    return sum(map(len, map(f, all_nodes(par))))


def part2(s: str) -> int:
    pass


def main():
    s = get_file(6)
    print(f"{part1(s)=}")
    print(f"{part2(s)=}")


if __name__ == "__main__":
    main()
