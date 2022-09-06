#![allow(dead_code)]

use std::fs;

pub type Num = i64;
pub type Program = Vec<Num>;
pub type Data = Vec<Num>;

use num_complex::Complex;
use std::collections::HashMap;
use std::collections::HashSet;

#[derive(PartialEq, Clone, Eq, Debug)]
pub struct ProgramState {
    program: Program,
    pointer: usize,
    relative_base: Num,
}

impl ProgramState {
    pub fn new(program: &Program, pointer: usize, relative_base: Num) -> ProgramState {
        ProgramState {
            program: program.to_vec(),
            pointer,
            relative_base,
        }
    }

    pub fn base(program: &Program) -> ProgramState {
        ProgramState::new(program, 0, 0)
    }
}

pub fn to_filename(day: Num) -> String {
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

fn to_grid<V>(data: &HashMap<Complex<Num>, V>) -> Vec<Vec<&V>> {
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

    grid
}

pub fn upside_down(data: &HashSet<Complex<Num>>) -> HashSet<Complex<Num>> {
    let mut res = HashSet::new();
    let ys: HashSet<_> = data.iter().map(|c| c.im).collect();
    if data.is_empty() {
        return res;
    }
    let ma = ys.iter().max().unwrap();
    let mi = ys.iter().min().unwrap();

    for c in data.iter() {
        let y = (ma - c.im) + mi;
        res.insert(Complex::new(c.re, y));
    }

    res
}

pub fn to_map(data: &HashSet<Complex<Num>>) -> HashMap<Complex<Num>, char> {
    let mut res = HashMap::new();

    if data.is_empty() {
        return res;
    }

    let ys: HashSet<_> = data.iter().map(|c| c.im).collect();
    let xs: HashSet<_> = data.iter().map(|c| c.re).collect();

    let top_left = Complex::new(*xs.iter().min().unwrap(), *ys.iter().min().unwrap());
    let bottom_right = Complex::new(*xs.iter().max().unwrap(), *ys.iter().max().unwrap());

    let size = bottom_right - top_left;

    for x in (0..=size.re) {
        for y in (0..=size.im) {
            let p = Complex::new(x, y);
            let contained = data.contains(&(p + top_left));

            let c = match contained {
                false => ' ',
                true => '#',
            };

            res.insert(p, c);
        }
    }

    res
}

pub fn s_display(data: &HashMap<Complex<Num>, char>) {
    for i in to_grid(data) {
        println!("{:}", i.iter().cloned().collect::<String>());
    }
    println!();
}

pub fn display<V: std::fmt::Debug>(data: &HashMap<Complex<Num>, V>) {
    for i in to_grid(data) {
        //  println!("{:}", i.iter().cloned().collect::<String>());
        println!("{:?}", i);
    }
    println!();
}

fn required_positions(n: Num) -> Num {
    match n {
        1 => 3,
        2 => 3,
        3 => 1,
        4 => 1,
        5 => 2,
        6 => 2,
        7 => 3,
        8 => 3,
        9 => 1,
        99 => 0,
        _ => panic!(),
    }
}

fn extend(mut p: Program, n: usize) -> Program {
    let mut k = vec![0; ((1 + (n as Num) - (p.len() as Num)).max(0)) as usize];
    p.append(&mut k);
    p
}

pub fn interpret(
    mut prog: ProgramState,
    input_: &Data,
    human_input: bool,
) -> (ProgramState, Option<Num>) {
    let mut data = prog.program;
    let mut i = prog.pointer;
    let mut relative_base = prog.relative_base;

    let mut input = input_.iter();

    let mut result = None;

    loop {
        let instruction = data[i];
        let mut params = instruction / 100;
        let op = instruction % 100;

        if op == 99 {
            break;
        };
        i += 1;

        let mut registers = Vec::new();
        for _ in 0..required_positions(op) - 1 {
            let d = params % 10;
            params /= 10;
            let p = data[i];

            let v = match d % 10 {
                0 => {
                    data = extend(data, p as usize);
                    data[p as usize]
                }
                1 => p as Num,
                2 => {
                    data = extend(data, (p + relative_base) as usize);
                    data[(p + relative_base) as usize]
                }

                _ => panic!(),
            };

            registers.push(v);
            i += 1
        }

        let mut p = data[i];

        let res = match params % 10 {
            0 => {
                data = extend(data, p as usize);
                data.get_mut(p as usize).unwrap()
            }
            1 => &mut p,
            2 => {
                data = extend(data, (p + relative_base) as usize);
                data.get_mut((p + relative_base) as usize).unwrap()
            }

            _ => panic!(),
        };

        match op {
            1 => *res = registers[0] + registers[1],
            2 => *res = registers[0] * registers[1],
            3 => {
                *res = if !human_input {
                    *input.next().unwrap()
                } else {
                    let mut line = String::new();
                    println!("input :");
                    std::io::stdin().read_line(&mut line).unwrap();
                    println!("{:?}", &line);

                    line.trim().parse::<Num>().unwrap()
                }
            }
            4 => result = Some(*res),
            5 => {
                if registers[0] != 0 {
                    i = *res as usize;
                    continue;
                }
            }
            6 => {
                if registers[0] == 0 {
                    i = *res as usize;
                    continue;
                }
            }
            7 => *res = (registers[0] < registers[1]) as Num,
            8 => *res = (registers[0] == registers[1]) as Num,
            9 => relative_base += *res,
            _ => panic!(),
        }
        i += 1;
        if result.is_some() {
            break;
        }
    }

    prog.program = data;
    prog.pointer = i;
    prog.relative_base = relative_base;

    (prog, result)
}
