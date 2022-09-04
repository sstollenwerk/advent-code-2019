from generic import get_file, parse_intcode, iterpret_intcode, cat_maybes


def part1(s: str) -> int:
    data = parse_intcode(s)
    data[1] = 12
    data[2] = 2
    res = int_incode(data)
    return res


def int_incode(data):
    return next(cat_maybes(iterpret_intcode(data)))


def part2(s: str) -> int:
    data_ = parse_intcode(s)
    for noun in range(100):
        for verb in range(100):
            data = data_.copy()
            data[1] = noun
            data[2] = verb
            if int_incode(data) == 19690720:
                return (100 * noun) + verb
    raise ValueError()


def main():
    s = get_file(2)
    print(f"{part1(s)=}")
    print(f"{part2(s)=}")


if __name__ == "__main__":
    main()
