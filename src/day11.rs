use crate::lib::get_data_program;
use crate::lib::{interpret, Data, Num, Program, ProgramState};

use crate::lib::{s_display, to_map, upside_down};

use std::collections::HashSet;

use num_complex::Complex;
type Position = Complex<Num>;
type Positions = HashSet<Position>;

fn rotation(n: Num) -> Position {
    match n {
        0 => Position::new(0, 1),
        1 => Position::new(0, -1),
        _ => panic!(),
    }
}

fn run(p: &Program, mut whites: Positions) -> (Positions, Positions) {
    let mut prog = ProgramState::base(p);

    let mut painted = Positions::new();

    let mut position = Position::new(0, 0);
    let mut direction = Position::new(0, 1);

    loop {
        let input = vec![whites.contains(&position) as Num];
        let (prog_, res) = interpret(prog, &input);
        prog = prog_;
        if let Some(n) = res {
            painted.insert(position);
            if n == 1 {
                whites.insert(position);
            } else {
                whites.remove(&position);
            }
        } else {
            break;
        }

        let input = vec![];
        let (prog_, res) = interpret(prog, &input);
        prog = prog_;
        let r = rotation(res.unwrap());
        direction *= r;
        position += direction
    }
    (painted, whites)
}

pub fn part1() -> usize {
    let prog = get_data_program(11);

    let (painted, _whites) = run(&prog, Positions::new());
    painted.len()
}

pub fn part2() {
    let prog = get_data_program(11);

    let mut whites_ = Positions::new();
    whites_.insert(Position::new(0, 0));

    let (_painted, whites) = run(&prog, whites_);

    s_display(&to_map(&upside_down(&whites)))
}
