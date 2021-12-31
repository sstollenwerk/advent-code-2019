#![feature(is_sorted)]
//#![feature(generators, generator_trait)]
// couldn't figure out how to pass data to the generator
#![allow(unused_parens)]

mod day07;
use day07::{part1, part2};
mod lib;

fn main() {
    println!("{:?}", part1());
    println!("{:?}", part2());
}
