from itertools import permutations
from functools import partial
from copy import deepcopy

from generic import get_file, parse_intcode, iterpret_intcode, cat_maybes, IntCode


def get_signal(program: IntCode, sequence: list[int]) -> int:
    signal = 0
    for s in sequence:
        comp = iterpret_intcode(deepcopy(program))
        next(comp)
        comp.send(s)
        signal = comp.send(signal)
    return signal


def get_looping_signal(program: IntCode, sequence: list[int]) -> int:
    sequence = list(sequence)
    comps = [iterpret_intcode(deepcopy(program)) for _ in sequence]
    for (s, c) in zip(sequence, comps):
        next(c)
        c.send(s)

    signals: list[int] = []
    signal = 0
    while True:
        for c in comps:
            try:
                s = c.send(signal)
            except StopIteration:
                return signals[-6]
                # for compatability with part 2, computer sends last data[0] before halting
            if s is not None:
                signal = s
                signals.append(signal)


def part1(s: str) -> int:
    code = parse_intcode(s)
    return max(map(partial(get_signal, code), permutations(range(5))))


def part2(s: str) -> int:
    code = parse_intcode(s)
    return max(map(partial(get_looping_signal, code), permutations(range(5, 10))))


def main():
    s = get_file(7)
    print(f"{part1(s)=}")
    print(f"{part2(s)=}")


if __name__ == "__main__":
    main()
