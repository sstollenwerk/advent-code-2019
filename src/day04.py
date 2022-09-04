from itertools import groupby

from collections.abc import Iterable

from generic import ilen, flist, get_file

password = list[int]


def c0(p: password) -> bool:
    return max(map(ilen, (v for k, v in groupby(p)))) >= 2


def c1(p: password) -> bool:
    return p == sorted(p)

def c2(p: password) -> bool:
    return 2 in (map(ilen, (v for k, v in groupby(p))))

def posses(s: str) -> Iterable[password]:
    base = range(*map(int, s.split("-")))
    return (list(map(int, str(i))) for i in base)


def valid1(p: password):
    return all(flist([c0, c1], p))

def valid2(p: password):
    return all(flist([c2, c1], p))


def part1(s: str) -> int:
    return ilen(filter(valid1, posses(s)))

def part2(s: str) -> int:
    return ilen(filter(valid2, posses(s)))

def main():
    s = get_file(4)
    print(f"{part1(s)=}")
    print(f"{part2(s)=}")


if __name__ == "__main__":
    main()
