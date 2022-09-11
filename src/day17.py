from math import prod


from generic import (
    get_file,
    parse_intcode,
    iterpret_intcode,
    cat_maybes,
    IntCode,
    Position,
    as_pos,
    map_vals,
    map_keys,
    as_display,
    rotate_counterclockwise,
    as_grid,
    fill,
    flatten,
)


def alignment_parameter(v: complex | Position) -> int:
    p = as_pos(v)
    return prod(p)


def find_intersections(vals: set[complex]) -> set[complex]:
    deltas = {1, -1, 1j, (-1j)}
    return {i for i in vals if (r := {a + i for a in deltas}) & vals == r}


def find_c(s: str, c: str) -> set[complex]:
    return {
        complex(i, j)
        for i, r in enumerate(s.split("\n"))
        for j, c_ in enumerate(r)
        if c_ == c
    }


def fake_control(s):
    direction = -1
    pos = find_c(s, "^").pop()
    grid = list(map(list, s.split()))

    moves = [
        ("L", 4),
        ("L", 10),
        ("L", 6),
        ("L", 4),
        ("L", 10),
        ("L", 6),
        ("L", 6),
        ("L", 4),
        ("R", 8),
        ("R", 8),
        ("L", 6),
        ("R", 8),
        ("L", 10),
        ("L", 8),
        ("L", 8),
        ("L", 4),
        ("L", 10),
        ("L", 6),
        ("L", 6),
        ("R", 8),
        ("L", 10),
        ("L", 8),
        ("L", 8),
        ("L", 6),
        ("L", 4),
        ("R", 8),
        ("R", 8),
        ("L", 6),
        ("R", 8),
        ("L", 10),
        ("L", 8),
        ("L", 8),
        ("L", 4),
        ("L", 10),
        ("L", 6),
        ("L", 6),
        ("L", 4),
        ("R", 8),
        ("R", 8),
    ]

    A = [
        ("L", 4),
        ("L", 10),
        ("L", 6),
    ]

    B = [
        ("L", 6),
        ("L", 4),
        ("R", 8),
        ("R", 8),
    ]
    C = [
        ("L", 6),
        ("R", 8),
        ("L", 10),
        ("L", 8),
        ("L", 8),
    ]

    moves_ = [
        A,
        A,
        B,
        C,
        A,
        C,
        B,
        C,
        A,
        B,
    ]
    #found manually via human pattern recognition and find+replace 

    assert moves == list(flatten(moves_))

    for i in moves:
        rot, move = i

        move = int(move)
        rot = {"L": 1j, "R": (-1j)}[rot]
        direction *= rot
        new_pos = pos + move * direction
        p, np = as_pos(pos), as_pos(new_pos)
        tmp = grid[p[0]][p[1]]
        print(tmp, grid[np[0]][np[1]])
        grid[p[0]][p[1]] = grid[np[0]][np[1]]
        grid[np[0]][np[1]] = tmp
        list(map(print, grid))
        pos = new_pos


def part1(s: str) -> int:
    code = parse_intcode(s)
    grid = iterpret_intcode(code)
    scaffolding = "".join(map(chr, grid))

    inters = find_intersections(find_c(scaffolding, "#"))

    return sum(map(alignment_parameter, inters))


def main_input() -> str:
    moves_ = [
        "A",
        "A",
        "B",
        "C",
        "A",
        "C",
        "B",
        "C",
        "A",
        "B",
    ]
    A = [
        ("L", 4),
        ("L", 10),
        ("L", 6),
    ]

    B = [
        ("L", 6),
        ("L", 4),
        ("R", 8),
        ("R", 8),
    ]

    C = [
        ("L", 6),
        ("R", 8),
        ("L", 10),
        ("L", 8),
        ("L", 8),
    ]
    sections = list(map(list, map(flatten, [A, B, C])))
    sections = [moves_] + sections

    sections = [",".join(list(map(str, i))) for i in sections]

    return "\n".join(sections)


def part2(s: str) -> int:
    code = parse_intcode(s)
    code[0] = 2
    input_ = list(map(ord, main_input() + "\nn\n"))
    print(input_)
    f = iter(input_).__next__
    comp = iterpret_intcode(code, f)

    return list(comp)[-2]
    ##fake_control(scaffolding)


def main():
    s = get_file(17)
    print(f"{part1(s)=}")
    print(f"{part2(s)=}")


if __name__ == "__main__":
    main()
