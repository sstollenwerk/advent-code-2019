from generic import get_file, parse_intcode, iterpret_intcode


def part1(s: str) -> int:
    data = parse_intcode(s)
    data[1] = 12
    data[2] = 2
    res = iterpret_intcode(data)
    return res


def part2(s: str) -> int:
    data_ = parse_intcode(s)
    for noun in range(100):
        for verb in range(100):
            data = data_.copy()
            data[1] = noun
            data[2] = verb
            if iterpret_intcode(data) == 19690720:
                return (100 * noun) + verb
    raise ValueError()


def main():
    s = get_file(2)
    print(f"{part1(s)=}")
    print(f"{part2(s)=}")


if __name__ == "__main__":
    main()
