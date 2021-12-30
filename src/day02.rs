use crate::lib::to_filename;

use std::fs;
use std::iter::successors;

type Num = usize;
type Program = Vec<Num>;

fn get_data() -> Program {
    fs::read_to_string(to_filename(2))
        .expect("Could not read file")
        .lines()
        .map(|s| read_row(s))
        .next()
        .unwrap()
}

fn read_row(row: &str) -> Program {
    row.split(',').map(|s| s.parse::<Num>().unwrap()).collect()
}

fn interpret(mut data: Program) -> Program {
    let mut i = 0;
    loop {


        let op = data[i];
        if op == 99 {
            break;
        };
        let (a, b, c) = (data[i + 1], data[i + 2], data[i + 3]);




        let (v1, v2) = (data[a], data[b]);
        data[c] = match op {
            1 => v1 + v2,
            2 => v1 * v2,
            _ => panic!(),
        };
        i += 4;
    }
    data
}

pub fn part1() -> Num {
    let mut vals = get_data();

    vals[1] = 12;
    vals[2] = 2;

    let res =  interpret(vals) ;

    println!("{:?}",res);
    res[0]
}

pub fn part2() -> Num {
    let vals = get_data();

    todo!();
}
