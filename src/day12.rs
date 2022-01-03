use std::cmp::Ordering;
use std::fs;

use crate::lib::Num;
use crate::lib::{display, s_display, to_filename};

use nalgebra::{Point3, Vector3};

//type Position = Point3<Num>;
type Position = Vector3<Num>;
// should be point but doesn't implement useful funcs

type Velocity = Vector3<Num>;

type Positions = Vec<Position>;
type Velocities = Vec<Velocity>;

type Particle = (Position, Velocity);

fn get_data() -> Vec<Position> {
    fs::read_to_string(to_filename(12))
        .expect("Could not read file")
        .lines()
        .map(|s| read_row(s))
        .collect()
}

fn read_row(row: &str) -> Position {
    let groups = row[1..row.len() - 1].split(',');
    let parts = groups.map(|s| s.split('=').nth(1).unwrap().parse::<Num>().unwrap());
    Position::from(Velocity::from_iterator(parts))
}

fn as_particle(p: Position) -> Particle {
    (p, Velocity::zeros())
}

fn deltas(vals: &Positions) -> Velocities {
    let mut res = Vec::new();

    for a in vals.iter() {
        let mut delta = Velocity::zeros();

        for b in vals.iter() {
            for d in 0..3 {
                let diff = match a[d].cmp(&b[d]) {
                    Ordering::Less => 1,
                    Ordering::Equal => 0,
                    Ordering::Greater => -1,
                };

                delta[d] += diff;
            }
        }
        res.push(delta);
    }

    res
}

fn step(mut positions: Positions, mut vels: Velocities) -> (Positions, Velocities) {
    let deltas = deltas(&positions);
    vels = vels
        .into_iter()
        .zip(deltas.into_iter())
        .map(|(a, b)| a + b)
        .collect();
    positions = positions
        .into_iter()
        .zip(vels.iter())
        .map(|(a, &b)| a + b)
        .collect();

    (positions, vels)
}

fn energy(p: Vector3<Num>) -> Num {
    (p.abs().sum())
}

fn total_energy(p: Particle) -> Num {
    energy(p.0) * energy(p.1)
}

pub fn part1() -> Num {
    let mut poses = get_data();
    println!("{:?}", poses);
    println!("");

    let mut vels: Velocities = poses.iter().map(|_| Velocity::zeros()).collect();

    for _ in (0..1000) {
        (poses, vels) = step(poses, vels);
        // println!("{:?}", poses);
        //   println!("{:?}", vels);
        //     println!("");
    }

    let particles = poses.into_iter().zip(vels.into_iter());
    particles.map(|p| total_energy(p)).sum()
}

pub fn part2() -> Num {
    todo!()
}
