use crate::lib::to_filename;

use std::fs;

use itertools::Itertools;
use num_complex::Complex;

use std::collections::HashSet;

type Num = i32;
type Position = Complex<Num>;
type Path = Vec<Position>;

type F = f64;

type Line = (Position, Position);

fn get_data() -> Vec<Path> {
    fs::read_to_string(to_filename(3))
        .expect("Could not read file")
        .lines()
        .map(|s| read_row(s))
        .collect()
}

fn read_row(row: &str) -> Path {
    row.split(',').map(|s| read_move(s)).collect()
}

fn read_move(row: &str) -> Position {
    let dir = match row.chars().nth(0).unwrap() {
        'R' => Position::new(1, 0),
        'U' => Position::new(0, 1),
        'L' => Position::new(-1, 0),
        'D' => Position::new(0, -1),
        _ => panic!(),
    };
    let delta = row[1..].parse::<Num>().unwrap();
    dir.scale(delta)
}

fn points(deltas: &Path) -> Path {
    deltas
        .iter()
        .scan(Position::new(0, 0), |state, &c| {
            *state += c;
            Some(*state)
        })
        .collect()
}

fn distances(deltas: &Path) -> Vec<Num> {
    deltas
        .iter()
        .scan(0, |state, &c| {
            *state += manhatten_dist(&c);
            Some(*state)
        })
        .collect()
}
fn intersection(a: &Line, b: &Line) -> Option<Position> {
    // https://en.wikipedia.org/wiki/Line–line_intersection#Given_two_points_on_each_line_segment

    let (x1, y1) = (a.0.re, a.0.im);
    let (x2, y2) = (a.1.re, a.1.im);
    let (x3, y3) = (b.0.re, b.0.im);
    let (x4, y4) = (b.1.re, b.1.im);

    let t_a = (x1 - x3) * (y3 - y4) - (y1 - y3) * (x3 - x4);
    let t_b = (x1 - x2) * (y3 - y4) - (y1 - y2) * (x3 - x4);

    let u_a = (x1 - x3) * (y1 - y2) - (y1 - y3) * (x1 - x2);
    let u_b = (x1 - x2) * (y3 - y4) - (y1 - y2) * (x3 - x4);

    let t = (t_a as F) / (t_b as F);

    let u = (u_a as F) / (u_b as F);

    if !((0.0 <= t && t <= 1.0) && (0.0 <= u && u <= 1.0)) {
        None
    } else {
        let x = x1 + ((t * ((x2 - x1) as F)).round() as Num);
        let y = y1 + ((t * ((y2 - y1) as F)).round() as Num);

        Some(Position::new(x, y))
    }
}

fn manhatten_dist(p: &Position) -> Num {
    p.re.abs() + p.im.abs()
}

pub fn part1() -> Num {
    let rows = get_data();
    let ax: Vec<Line> = points(&rows[0]).into_iter().tuple_windows().collect();
    let bx: Vec<Line> = points(&rows[1]).into_iter().tuple_windows().collect();

    let mut intersections = Vec::new();

    for a in ax.iter() {
        for b in bx.iter() {
            if let Some(p) = intersection(a, b) {
                intersections.push(p);
            }
        }
    }

    intersections.iter().map(manhatten_dist).min().unwrap()
}

pub fn part2() -> Num {
    let rows = get_data();
    let ax: Vec<Line> = points(&rows[0]).into_iter().tuple_windows().collect();
    let ad: Vec<Num> = distances(&rows[0]);
    let bx: Vec<Line> = points(&rows[1]).into_iter().tuple_windows().collect();
    let bd: Vec<Num> = distances(&rows[1]);

    let mut intersections = Vec::new();
    for (a, da) in ax.iter().zip(ad.iter()) {
        for (b, db) in bx.iter().zip(bd.iter()) {
            if let Some(p) = intersection(a, b) {
                let d1 = manhatten_dist(&(p - a.0)) + *da;
                let d2 = manhatten_dist(&(p - b.0)) + *db;
                intersections.push(d1 + d2);
            }
        }
    }

    *intersections.iter().min().unwrap()
}
