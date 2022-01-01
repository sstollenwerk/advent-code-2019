use crate::lib::Num;
use crate::lib::{display, s_display, to_filename};

use std::fs;

use core::ops::Deref;
use std::slice::Iter;

use fraction::GenericFraction;
use itertools::Itertools;
use num_complex::Complex;
use ordered_float::OrderedFloat;

use std::collections::HashSet;
use std::collections::VecDeque;

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

fn polar(p: Point) -> (f64, f64) {
    let p_ = Complex::new(p.re as f64, p.im as f64);

    p_.to_polar()
}

fn equivalence_classes<'a, T: 'a + Copy, G>(vals: impl Iterator<Item = &'a T>, f: G) -> Vec<Vec<T>>
where
    G: Fn(T, T) -> bool,
{
    let mut data: Vec<Vec<T>> = Vec::new();
    for el in vals {
        let mut seen = false;

        for (i, g) in data.iter().enumerate() {
            if f(g[0], *el) {
                let mut g_ = g.clone();
                g_.push(*el);
                data[i] = g_;
                seen = true;
                break;
            }
        }

        if !seen {
            data.push(vec![*el])
        }
    }

    data
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

    /*
        polar(a).1 == polar(b).1

        didn't work. Not sure whether alg was wrong or f64 is insufficient. Assuminmg latter
    */
}

fn block(a: Point, b: Point) -> bool {
    // should check if b is blocked by a
    // not if a is blocked by b
    let origin = Point::new(0, 0);
    (a != origin) && (b != origin) && colinear(a, b) && (b.l1_norm() > a.l1_norm())
}

fn groups(p: Point, asts: &Positions) -> Vec<Vec<Point>> {
    let mut deltas: Positions = asts.iter().map(|&c| c - p).collect();
    deltas.remove(&Point::new(0, 0));

    equivalence_classes(deltas.iter(), colinear)
}

fn ast_order(p: Point, asts: &Positions) -> Vec<Point> {
    let g_ = groups(p, asts);
    let mut g = Vec::new();
    for (i, el_) in g_.iter().enumerate() {
        let mut el = el_.to_vec();
        el.sort_by_key(|c| c.l1_norm());
        el.reverse();
        g.push(el);
    }

    g.sort_by_key(|cx| {
        OrderedFloat(
            (std::f64::consts::TAU + polar(cx[0]).1 - polar(Point::new(0, -1)).1)
                % std::f64::consts::TAU,
        )
    });

    let mut q = VecDeque::new();
    for i in g.into_iter() {
        q.push_back(i);
    }

    let mut res = Vec::new();
    while q.len() > 0 {
        let mut n = q.pop_front().unwrap();
        let val = n.pop().unwrap() + p;

        res.push(val);

        if n.len() > 0 {
            q.push_back(n);
        }
    }

    res
}

fn see(p: Point, asts: &Positions) -> usize {
    groups(p, asts).len()
}

pub fn part1() -> usize {
    let data = get_data();

    data.iter().map(|c| see(*c, &data)).max().unwrap()
}

pub fn part2() -> Num {
    let data = get_data();

    let pos = *data.iter().max_by_key(|&&c| see(c, &data)).unwrap();

    println!("{:?}", pos);

    let nth =  ast_order(pos, &data)[199] ;

    (100*nth.re + nth.im)

}
