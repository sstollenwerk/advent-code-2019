from generic import get_file

from itertools import accumulate, starmap, product

delta = complex
point = complex
wire = tuple[point, point]


def asdelta(s: str) -> delta:
    d, xs = s[0], s[1:]
    amt = int(xs)
    direction = {"U": 1j, "R": 1, "L": -1, "D": -1j}[d]
    return direction * amt


# accumulate is [a] -> (b -> a -> b )
def to_wire(w: wire, d: delta) -> wire:
    (a, b) = w
    return b, b + d


def is_horiz(w: wire):
    return w[0].real == w[1].real


def intersection(a: wire, b: wire) -> point | None:
    # where does (0,0) -> (0,5) intersect with (8,7) -> (5,7)? (0,7) which isn't on line.
    # where does (0,0) -> (0,5) intersect with (8,7) -> (8,9)? Nowhere

    if is_horiz(a) == is_horiz(b):
        return None
    if is_horiz(b):
        return intersection(b, a)

    intersect = a[0].real + (b[0].imag) * 1j
    assert intersect == a[1].real + (b[1].imag) * 1j
    p1, p2 = sorted([a[0].imag, a[1].imag])
    k1, k2 = sorted([b[0].real, b[1].real])
    if p1 <= intersect.imag <= p2 and k1 <= intersect.real <= k2:
        return intersect
    return None


def wires(deltas: list[delta]) -> list[wire]:
    return list(accumulate(deltas, to_wire, initial=(0, 0)))


def manhatten(p: point) -> float:
    return abs(p.real) + abs(p.imag)


def part1(s: str) -> int:
    wires_ = [wires(list(map(asdelta, W.split(',')))) for W in s.split()]
    crosses = filter(bool, starmap(intersection, product(*wires_)))
    return round(min(map(manhatten, crosses)))


def part2(s: str) -> int:
    pass


def main():
    s = get_file(3)
    print(f"{part1(s)=}")
    print(f"{part2(s)=}")


if __name__ == "__main__":
    main()
