from more_itertools import nth

from generic import (
    get_file,
    iterate,
)

position = int
velocity = int

MoonAxis = tuple[position, velocity]

Moon = tuple[MoonAxis, MoonAxis, MoonAxis]


def cmp(a, b) -> int:
    # from https://stackoverflow.com/questions/22490366/how-to-use-cmp-in-python-3
    return (a > b) - (a < b)


def parse(s: str) -> list[list[MoonAxis]]:
    rows = s.replace(">", ",").replace("=", ",").split("\n")
    ind_moons = [list(map(int, r.split(",")[1::2])) for r in rows]
    axes = list(zip(*ind_moons))

    return [[(p, 0) for p in r] for r in axes]


def step_single_axis(moons: list[MoonAxis]) -> MoonAxis:
    amts = list(map(list, moons))
    for i, a in enumerate(moons):
        for j, b in enumerate(moons):
            amts[i][1] -= cmp(a[0], b[0])
    new_state = [(p + v, v) for [p, v] in amts]
    return new_state


def potential_energy(m: Moon) -> int:
    positions, velocities = zip(*m)
    return sum(map(abs, positions)) * sum(map(abs, velocities))


def all_potent_energy(axes: list[list[MoonAxis]]) -> int:
    return sum(map(potential_energy, zip(*axes)))


def part1(s: str) -> int:
    positions = [nth(iterate(step_single_axis, ax), 1000) for ax in parse(s)]
    return all_potent_energy(positions)


def part2(s: str) -> int:
    pass


def main():
    s = get_file(12)

    print(f"{part1(s)=}")
    print(f"{part2(s)=}")


if __name__ == "__main__":
    main()
