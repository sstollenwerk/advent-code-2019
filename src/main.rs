#![feature(is_sorted)]
#![allow(unused_parens)]

mod day08;
use day08::{part1, part2};
mod lib;

fn main() {
    println!("{:?}", part1());
    println!("{:?}", part2());
}
