use crate::lib::Num;
use crate::lib::{display, s_display, to_filename};

use std::fs;

use fraction::GenericFraction;
use itertools::Itertools;
use num_complex::Complex;

use std::collections::HashSet;

type Point = Complex<Num>;

type Positions = HashSet<Point>;

type F = GenericFraction<Num>;

fn get_data() -> Positions {
    let mut res = Positions::new();

    for (y, row) in fs::read_to_string(to_filename(10))
        .expect("Could not read file")
        .lines()
        .map(read_row)
        .enumerate()
    {
        for (x, c) in row.into_iter().enumerate() {
            if c {
                res.insert(Point::new(x.try_into().unwrap(), y.try_into().unwrap()));
            }
        }
    }
    res
}

fn read_row(row: &str) -> Vec<bool> {
    row.chars()
        .map(|c| match c {
            '.' => false,
            '#' => true,
            _ => panic!(),
        })
        .collect()
}

fn colinear(a: Point, b: Point) -> bool {
    let scale = if a.re != 0 {
        F::new(b.re, a.re)
    } else {
        F::new(b.im, a.im)
    };

    (a.re.signum() == b.re.signum())
        && (a.im.signum() == b.im.signum())
        && ((F::new(a.im, 1) * scale) == F::new(b.im, 1))
        && ((F::new(a.re, 1) * scale) == F::new(b.re, 1))
}

fn block(a: Point, b: Point) -> bool {
    // should check if b is blocked by a
    // not if a is blocked by b
    let origin = Point::new(0, 0);
    (a != origin) && (b != origin) && colinear(a, b) && (b.l1_norm() > a.l1_norm())
}

fn see(p: Point, asts: &Positions) -> usize {
    let mut deltas: Positions = asts.iter().map(|&c| c - p).collect();
    deltas.remove(&Point::new(0, 0));

    let blocked = deltas
        .iter()
        .cartesian_product(deltas.iter())
        .filter(|(a, b)| block(**a, **b))
        .map(|(_, b)| *b);

    deltas.len() - blocked.count()
}

pub fn part1() -> usize {
    let data = get_data();
    println!("{:?}", data);

    data.iter().map(|c| see(*c, &data)).max().unwrap()
}

pub fn part2() -> Num {
    todo!();
}
