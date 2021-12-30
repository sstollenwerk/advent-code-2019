use crate::lib::to_filename;

use std::fs;
use std::iter::successors;

type Num = i32;

fn get_data() -> Vec<Num> {
    fs::read_to_string(to_filename(1))
        .expect("Could not read file")
        .lines()
        .map(|s| read_row(s))
        .collect()
}

fn read_row(row: &str) -> Num {
    row.parse::<Num>().unwrap()
}

fn required_fuel(n: Num) -> Num {
    n / 3 - 2
}

fn part_req_fuel(n: &Num) -> Option<Num> {
    let k = required_fuel(*n);
    if k <= 0 {
        None
    } else {
        Some(k)
    }
}

fn total_required_fuel(n: Num) -> Num {
    let fuels = successors(Some(n), part_req_fuel);
    fuels.sum::<Num>() - n
}

pub fn part1() -> Num {
    let vals = get_data();

    vals.into_iter().map(|x| required_fuel(x)).sum()
}

pub fn part2() -> Num {
    let vals = get_data();

    vals.into_iter().map(|x| total_required_fuel(x)).sum()
}
