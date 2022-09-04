from typing import Callable, TypeVar
from collections.abc import Iterable

T = TypeVar("T")
IntCode = list[int]

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
    return list(map(int, s.split(",")))


def iterpret_intcode(xs_: IntCode) -> int:
    xs = xs_.copy()
    i = 0
    while True:

        a, b, c, *ignore = xs[i + 1 :]
        match xs[i]:
            case 1:

                xs[c] = xs[a] + xs[b]
            case 2:
                xs[c] = xs[a] * xs[b]
            case 99:
                return xs[0]
            case _:
                raise ValueError(f"{i=}, {xs[i]=},{xs=}")
        i += 4
