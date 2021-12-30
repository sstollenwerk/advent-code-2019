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

fn total_required_fuel(n: Num) -> Num {
    successors(Some(n), |n| Some(required_fuel(*n)))
        .take_while(|&x| x > 0)
        .skip(1)
        .sum()
}

pub fn part1() -> Num {
    let vals = get_data();

    vals.into_iter().map(|x| required_fuel(x)).sum()
}

pub fn part2() -> Num {
    let vals = get_data();

    vals.into_iter().map(|x| total_required_fuel(x)).sum()
}
