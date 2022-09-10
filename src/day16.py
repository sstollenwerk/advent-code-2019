from itertools import cycle, accumulate
from operator import mul
from collections.abc import Iterable

from more_itertools import nth

from generic import (
    get_file,
    iterate,
    map_vals,
    T,
    flatten,
)


Signal = list[int]

def replicate(n,i):
    return (i for _ in range(n))

def parse(s: str) -> Signal:
    return list(map(int, s))


def part(sig: Signal, n: int) -> int:
    pat = make_pattern(n)
    r = sum(map(mul, sig, pat))
    return abs(r) % 10


def make_pattern(n: int) -> Iterable[int]:
    base = [0, 1, 0, -1]
    r = cycle(flatten(replicate(n,i) for i in base))
    next(r)
    return r


def step(s: Signal) -> Signal:
    return [part(s, i + 1) for i in range(len(s))]



def part1(s):
    signal = parse(s)
    res = nth(iterate(step, signal), 100)
    return int(as_str(res)[:8])

def as_str(s:Signal) -> str:
    return "".join(map(str, s))

def last_digits(signal:Signal) -> Signal:
    part = accumulate(signal[::-1])
    return [abs(n)%10 for n in list(part)[::-1]]
    

def part2(s):
    # needed to look up help for this one - https://www.reddit.com/r/adventofcode/comments/ebf5cy/comment/fb4bvw4/
    # disappointing that this doesn't work where offset is in first half of signal.
    signal = parse(s*10000)
    offset = int(as_str(signal)[:7])
    signal = signal[offset:]
    res = nth(iterate(last_digits, signal), 100)
    print(res[:100])
    return int(as_str(res)[0:0+8])


def main():
    s = get_file(16).strip()
    print(f"{part1(s)=}")
    print(f"{part2(s)=}")


if __name__ == "__main__":
    main()
