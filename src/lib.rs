use std::fs;

pub type Num = i32;
pub type Program = Vec<Num>;
pub type Data = Vec<Num>;

use num_complex::Complex;
use std::collections::HashMap;

pub fn to_filename(day: i32) -> String {
    format!("input/{:0>2}.txt", day)
}

pub fn get_data_program(n: Num) -> Program {
    fs::read_to_string(to_filename(n))
        .expect("Could not read file")
        .lines()
        .map(read_program)
        .next()
        .unwrap()
}

fn read_program(row: &str) -> Program {
    row.split(',').map(|s| s.parse::<Num>().unwrap()).collect()
}

pub fn display<V: std::fmt::Debug>(data: &HashMap<Complex<Num>, V>) {
    let largest = data.keys().map(|c| (c.re, c.im)).max().unwrap();

    let mut grid = Vec::new();

    for y in (0..=largest.1) {
        let mut res = Vec::new();

        for x in (0..=largest.0) {
            let d = &data[&Complex::new(x as Num, y as Num)];
            res.push(d);
        }
        grid.push(res);
    }

    for i in grid {
        //  println!("{:}", i.iter().cloned().collect::<String>());
        println!("{:?}", i);
    }
    println!();
}
