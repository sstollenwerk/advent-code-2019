from typing import Callable, TypeVar
from collections.abc import Iterable
from collections import defaultdict

T = TypeVar("T")
V = TypeVar("V")
IntCode = defaultdict[int, int]

toSameType = Callable[[T], T]


def get_file(n: int):
    dir = "../input/" + str(n).zfill(2) + ".txt"
    with open(dir, "r") as f:
        return f.read()


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


def get_vals(params: int, command: int, xs: IntCode, i: int) -> Iterable[int]:
    required_posses = {1: 3, 2: 3, 3: 1, 4: 1, 5: 2, 6: 2, 7: 3, 8: 3, 99: 0}
    required_vals = required_posses.get(command, 0)
    for k in range(i, i + required_vals):
        params, mode = divmod(params, 10)
        v = xs[k]
        if k == i + required_vals - 1 and command not in {5, 6}:
            yield v
            continue
            # this section is a pain
        match mode:
            case 0:
                yield xs[v]
            case 1:
                yield v


def iterpret_intcode(xs_: IntCode) -> Iterable[int | None]:
    xs = xs_.copy()
    i = 0
    while True:
        params, command = divmod(xs[i], 100)
        i += 1
        vals = list(get_vals(params, command, xs, i))
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
                yield xs[a]
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
