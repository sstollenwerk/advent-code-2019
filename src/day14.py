from collections import defaultdict, Counter
from copy import deepcopy
from math import ceil


from generic import (
    get_file,
    iterate,
    map_vals,
    T,
    flatten,
)


Mineral = str

Quant = tuple[Mineral, int]

Requires = dict[Mineral, tuple[int, list[Quant]]]


def as_quant(s: str) -> Quant:
    [a, b] = s.strip().split()
    return (b, int(a))


def parse(s: str) -> Requires:
    res = dict()
    for r in s.strip().split("\n"):
        *in_, out = list(map(as_quant, r.replace("=>", ",").split(",")))
        mineral, amt = out
        assert mineral not in res.keys()
        res[mineral] = (amt, in_)
    return dict(res)


def without_amts(r: Requires) -> dict[Mineral, list[Mineral]]:
    return map_vals(lambda x: [i[0] for i in x[1]], r)


def topological_sort(d_: dict[T, list[T]]) -> list[T]:
    # https://en.wikipedia.org/wiki/Topological_sorting#Kahn's_algorithm

    d = deepcopy(d_)

    res = []

    total = set(d.keys()) | set(flatten(d.values()))

    while S := set(d.keys()) - set(flatten(d.values())):
        n = min(S)
        del d[n]
        res.append(n)

    if d:
        raise ValueError

    remaining = total - set(res)

    res += sorted(remaining)

    return res


def find_required(r: Requires, q: Quant) -> Counter[Mineral]:
    order = topological_sort(without_amts(r))
    required = Counter(dict([q]))
    for o in order[:-1]:
        amt, takes = r.get(o, (0, []))
        req = required[o]
        for (mineral, a) in takes:
            required[mineral] += a * ceil(req / amt)
        ##del required[o]
    return required


def part1(s) -> int:
    graph = parse(s)
    order = topological_sort(without_amts(graph))
    needed = ("FUEL", 1)
    return find_required(graph, needed)["ORE"]


def part2(s) -> int:
    pass


def main():
    s = get_file(14)
    print(f"{part1(s)=}")
    print(f"{part2(s)=}")


if __name__ == "__main__":
    main()
