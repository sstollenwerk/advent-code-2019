from collections import defaultdict
from functools import partial
from itertools import groupby

from typing import Callable, TypeVar
from collections.abc import Iterable

T = TypeVar("T")
V = TypeVar("V")
U = TypeVar("U")
IntCode = defaultdict[int, int]

toSameType = Callable[[T], T]

Position = tuple[int, int]


def bin_items(f: Callable[[T], V], xs: list[T]) -> defaultdict[V, list[T]]:
    d = defaultdict(list)
    for x in xs:
        d[f(x)].append(x)
    return d


def delta(a: Position, b: Position) -> Position:
    x1, y1 = a
    x2, y2 = b
    return (x1 - x2, y1 - y2)


def tup_add(a: Position, b: Position) -> Position:
    a = complex(*a)
    b = complex(*b)
    return as_pos(a + b)


def get_file(n: int):
    dir = "../input/" + str(n).zfill(2) + ".txt"
    with open(dir, "r") as f:
        return f.read()


def rotate_counterclockwise(p: Position) -> Position:
    c = complex(*p)
    c *= 0 + 1j
    return as_pos(c)


def rotate_clockwise(p: Position) -> Position:
    c = complex(*p)
    c *= 0 - 1j
    return as_pos(c)


def as_pos(c: complex | Position) -> Position:
    if isinstance(c, complex):

        return (round(c.real), round(c.imag))
    return c


def transpose(p: Position) -> Position:
    a, b = p
    return (b, a)


def map_vals(f: Callable[[T], V], d: dict[U, T]) -> dict[U, V]:
    return {k: f(v) for k, v in d.items()}


def map_keys(f: Callable[[T], V], d: dict[T, U]) -> dict[V, U]:
    return {f(k): v for k, v in d.items()}


def as_display(data: set[complex | Position] | dict[complex | Position, bool]) -> str:
    if isinstance(data, dict):
        data = {k for k, v in data.items() if v}
    s = set(map(as_pos, data))
    disp = [" ", "*"].__getitem__
    d = to_d(s)

    d = map_vals(disp, d)
    grid = as_grid(d)

    r = "\n".join("".join(r) for r in grid)

    return r


def to_d(s_: set[complex | Position]) -> dict[Position, bool]:
    s = set(map(as_pos, s_))
    a = {i[0] for i in s}
    b = {i[1] for i in s}
    posses = (
        (i, j) for i in range(min(a), max(a) + 1) for j in range(min(b), max(b) + 1)
    )
    return {p: p in s for p in posses}


def as_grid(d: dict[complex | Position, T]) -> list[list[T]]:

    d = map_keys(as_pos, d)
    rows = [list(v) for k, v in groupby(sorted(d.keys()), lambda x: x[0])]

    res = [[d[i] for i in r] for r in rows]

    return res


def iterate(f: toSameType, x: T) -> Iterable[T]:
    """implement
    https://hackage.haskell.org/package/base-4.17.0.0/docs/Prelude.html#v:iterate
    iterate f x == [x, f x, f (f x), ...]
    """
    while True:
        yield x
        x = f(x)


def parse_intcode(s: str) -> IntCode:
    return defaultdict(int, enumerate(map(int, s.split(","))))


def get_vals(
    params: int, command: int, xs: IntCode, i: int, rel_base: int
) -> Iterable[int]:
    required_posses = {1: 3, 2: 3, 3: 1, 4: 1, 5: 2, 6: 2, 7: 3, 8: 3, 9: 1, 99: 0}
    required_vals = required_posses.get(command, 0)
    for k in range(i, i + required_vals):
        params, mode = divmod(params, 10)
        v = xs[k]
        if k == i + required_vals - 1 and command not in {4, 5, 6, 9}:
            match mode:
                case 0 | 1:
                    yield v
                case 2:
                    yield v + rel_base

            continue
            # this section is a pain
        match mode:
            case 0:
                yield xs[v]
            case 1:
                yield v

            case 2:
                yield xs[v + rel_base]


def iterpret_intcode(xs_: IntCode) -> Iterable[int | None]:
    xs = xs_.copy()
    i = 0
    rel_base = 0
    while True:
        params, command = divmod(xs[i], 100)
        i += 1
        vals = list(get_vals(params, command, xs, i, rel_base))
        jump = False
        match command:
            case 1:
                a, b, c = vals
                xs[c] = a + b
            case 2:
                a, b, c = vals
                xs[c] = a * b
            case 3:
                a = vals[0]
                r = yield None
                xs[a] = r
            case 4:
                a = vals[0]
                yield a
            case 5:
                a, b = vals
                if bool(a):
                    i = b
                    jump = True
            case 6:
                a, b = vals
                if not bool(a):
                    i = b
                    jump = True
            case 7:
                a, b, c = vals
                xs[c] = int(a < b)
            case 8:
                a, b, c = vals
                xs[c] = int(a == b)

            case 9:
                a = vals[0]
                rel_base += a
            case 99:
                # print(xs)
                yield xs[0]
                return
            case n:
                raise ValueError(f"{i=}, {n=}, {xs[i]=},{xs=}")

        if not jump:
            i += len(vals)


def flist(fs: list[Callable[[T], V]], x: T) -> list[V]:
    return [f(x) for f in fs]


def ilen(xs: Iterable[T]) -> int:
    return sum(1 for _ in xs)


def cat_maybes(xs: Iterable[T | None]) -> Iterable[T]:
    """port of
    https://hackage.haskell.org/package/base-4.15.0.0/docs/Data-Maybe.html#v:catMaybes
    """
    return (i for i in xs if i is not None)


def uncurry(f):
    def inner(t):
        return f(*t)

    return inner
