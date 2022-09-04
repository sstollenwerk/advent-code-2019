from typing import Callable, TypeVar
from collections.abc import Iterable

T = TypeVar("T")

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
