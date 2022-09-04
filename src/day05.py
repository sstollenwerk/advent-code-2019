from generic import get_file, parse_intcode, iterpret_intcode, cat_maybes


def part1(s: str) -> int:
    data = parse_intcode(s)
    r = iterpret_intcode(data)
    print(next(r))
    print(r.send(1))
    return list(cat_maybes(r))[:-1]


def part2(s: str) -> int:
    data = parse_intcode(s)
    r = iterpret_intcode(data)
    next(r)
    return r.send(5)


def main():
    s = get_file(5)
    print(f"{part1(s)=}")
    print(f"{part2(s)=}")


if __name__ == "__main__":
    main()
