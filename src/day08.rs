use crate::lib::Num;
use crate::lib::{display, s_display, to_filename};

use std::fs;

use counter::Counter;
use num_complex::Complex;

use std::collections::HashMap;

type Position = Complex<Num>;

type Layer = HashMap<Position, Num>;

type Image = Vec<Layer>;

fn get_data() -> Vec<Num> {
    fs::read_to_string(to_filename(8))
        .expect("Could not read file")
        .lines()
        .map(read_row)
        .next()
        .unwrap()
}

fn read_row(row: &str) -> Vec<Num> {
    row.chars()
        .map(|c| c.to_digit(10).unwrap() as Num)
        .collect()
}

fn make_layer(data: &[Num], size: Position) -> Layer {
    let mut res = Layer::new();
    for (i, el) in data.iter().enumerate() {
        let i = i as Num;
        let x = i % size.re;
        let y = i / size.re;
        let pos = Position::new(x, y);
        res.insert(pos, *el);
    }

    res
}

fn make_image(data: &[Num], size: Position) -> Image {
    data.chunks((size.re * size.im) as usize)
        .map(|c| make_layer(c, size))
        .collect()
}

fn get_image() -> Image {
    let size = Position::new(25, 6);
    let raw = get_data();
    make_image(&raw, size)
}

fn rendered(im: Image) -> Layer {
    let mut base = im[0].clone();
    let keys = im[0].keys();

    for c in keys {
        let mut i = 0;
        while im[i][c] == 2 {
            i += 1;
        }
        base.insert(*c, im[i][c]);
    }
    base
}

fn format(im: Layer) -> HashMap<Position, char> {
    let mut res = HashMap::new();
    for (k, val) in im.iter() {
        let v = match val {
            0 => ' ',
            1 => '*',
            _ => panic!(),
        };
        res.insert(*k, v);
    }
    res
}

pub fn part1() -> Num {
    let image = get_image();

    //  for i in image.iter(){
    //      display(i)
    //  }

    let amounts = image.iter().map(|c| c.values().collect::<Counter<_>>());
    let best = amounts.min_by_key(|x| x[&0]).unwrap();
    (best[&1] * best[&2]).try_into().unwrap()
}

pub fn part2() {
    let image = get_image();

    let r = rendered(image);

    s_display(&format(r))
}
