use crate::lib::Num;
use crate::lib::{display, s_display, to_filename};

use std::fs;

use counter::Counter;
use num_complex::Complex;

use std::collections::HashSet;

type Point = Complex<Num>;

type Positions = HashSet<Point>;

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
    // should check if b is blocked by a
    // not if a is blocked by b

    let scale = (b.re as f64) / (a.re as f64);

    (a.re.signum() == b.re.signum())
        && (a.im.signum() == b.im.signum())
        && ((a.im as f64 * scale).round() as Num == b.im)
}

fn block(a: Point, b: Point) -> bool {
    // should check if b is blocked by a
    // not if a is blocked by b
    colinear(a, b) && (b.re.abs() > a.re.abs())
}

fn testing() {
    let places = vec![
        Point::new(1, 2),
        Point::new(2, 4),
        Point::new(-2, -4),
        Point::new(2, 5),
    ];

    let vals = vec![
        block(places[0], places[1]),
        block(places[2], places[1]),
        block(places[1], places[0]),
        block(places[0], places[3]),
    ];

    let vals2 = vec![
        colinear(places[0], places[1]),
        colinear(places[2], places[1]),
        colinear(places[1], places[0]),
        colinear(places[0], places[3]),
    ];

    println!("{:?}", vals);
    println!("{:?}", vals2);
}

pub fn part1() -> Num {
    let data = get_data();
    println!("{:?}", data);
    todo!();
}

pub fn part2() -> Num {
    todo!();
}
