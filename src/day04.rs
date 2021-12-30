use crate::lib::to_filename;

use std::fs;

use counter::Counter;

type Num = u32;

fn get_data() -> impl Iterator<Item = Num> {
    fs::read_to_string(to_filename(4))
        .expect("Could not read file")
        .lines()
        .map(|s| read_row(s))
        .next()
        .unwrap()
}

fn read_row(row: &str) -> impl Iterator<Item = Num> {
    let vals: Vec<_> = row.split('-').map(|s| s.parse::<Num>().unwrap()).collect();
    (vals[0]..=vals[1])
}

fn valid_old(n: &Num) -> bool {
    let digits = n.to_string();
    let sorted_ = digits.chars().is_sorted();

    let dupes = digits
        .chars()
        .collect::<Counter<_>>()
        .values()
        .max()
        .unwrap()
        >= &2;

    sorted_ && dupes
}

fn valid(n: &Num) -> bool {
    let digits = n.to_string();
    let sorted_ = digits.chars().is_sorted();

    let double = digits
        .chars()
        .collect::<Counter<_>>()
        .values()
        .collect::<Vec<_>>()
        .contains(&&2);

    sorted_ && double
}

pub fn part1() -> usize {
    let posses: Vec<_> = get_data().filter(valid_old).collect();
    posses.len()
}

pub fn part2() -> usize {
    let posses: Vec<_> = get_data().filter(valid).collect();
    posses.len()
}
