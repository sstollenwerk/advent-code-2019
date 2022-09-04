from generic import get_file, parse_intcode, iterpret_intcode, cat_maybes


def part1(s: str) -> int:
    data = parse_intcode(s)
    r = iterpret_intcode(data)
    a = next(r)
    k = r.send(1)
    res = list(cat_maybes([a, k])) + list(cat_maybes(r))
    print(res)
    return res[0]


def part2(s: str) -> int:
    data = parse_intcode(s)
    r = iterpret_intcode(data)
    a = next(r)
    k = r.send(2)
    res = list(cat_maybes([a, k])) + list(cat_maybes(r))
    print(res)
    return res[0]


def main():
    s = get_file(9)
    print(f"{part1(s)=}")
    print(f"{part2(s)=}")


if __name__ == "__main__":
    main()
