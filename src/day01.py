from itertools import takewhile

from generic import get_file, iterate


def section(n: int) -> int:
    return max((n // 3) - 2, 0)


def full_section(n: int) -> int:
    return sum(takewhile(bool, iterate(section, n))) - n


def part1(s: str) -> int:
    return sum(map(section, map(int, s.split())))


def part2(s: str) -> int:
    return sum(map(full_section, map(int, s.split())))


def main():
    s = get_file(1)
    print(f"{part1(s)=}")
    print(f"{part2(s)=}")


if __name__ == "__main__":
    main()
