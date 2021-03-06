use crate::lib::get_data_program;
use crate::lib::{Data, Num, Program};

use itertools::Itertools;

#[derive(PartialEq, Clone, Eq, Debug)]
struct ProgramState {
    program: Program,
    pointer: usize,
}

impl ProgramState {
    fn new(program: &Program, pointer: usize) -> ProgramState {
        ProgramState {
            program: program.to_vec(),
            pointer,
        }
    }
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
        99 => 0,
        _ => panic!(),
    }
}

fn interpret_old(mut data: Program, input_: &Data) -> Data {
    let mut input = input_.iter();
    let mut i = 0;
    let mut output = Vec::new();
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
            let v = if d == 0 { data[p as usize] } else { p };
            registers.push(v);
            i += 1
        }

        let mut p = data[i];

        let res = if params % 10 == 0 {
            data.get_mut(p as usize).unwrap()
        } else {
            &mut p
        };

        match op {
            1 => *res = registers[0] + registers[1],
            2 => *res = registers[0] * registers[1],
            3 => *res = *input.next().unwrap(),
            4 => output.push(*res),
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
            _ => panic!(),
        }
        i += 1;
    }

    output
}

fn interpret(prog: &ProgramState, input_: &Data) -> (ProgramState, Option<Num>) {
    let mut data = prog.program.to_vec();
    let mut i = prog.pointer;

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
            let v = if d == 0 { data[p as usize] } else { p };
            registers.push(v);
            i += 1
        }

        let mut p = data[i];

        let res = if params % 10 == 0 {
            data.get_mut(p as usize).unwrap()
        } else {
            &mut p
        };

        match op {
            1 => *res = registers[0] + registers[1],
            2 => *res = registers[0] * registers[1],
            3 => *res = *input.next().unwrap(),
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
            _ => panic!(),
        }
        i += 1;
        if result.is_some() {
            break;
        }
    }

    let prog = ProgramState::new(&data, i);
    (prog, result)
}

fn thruster_signal(phases: &[Num], program: &Program) -> Num {
    let mut signal = 0;

    for i in phases.iter() {
        let vals = vec![*i, signal];
        signal = interpret_old(program.to_vec(), &vals)[0];
    }
    signal
}

fn feedback_thruster_signal(phases: &[Num], program: &Program) -> Num {
    let mut signal = 0;

    let mut states = vec![ProgramState::new(program, 0); 5];

    let mut i = 0;

    for el in phases.iter() {
        let vals = vec![*el, signal];
        let (state, signal_) = interpret(&states[i], &vals);
        states[i] = state;
        signal = signal_.unwrap();
        i = (i + 1) % states.len();
    }

    loop {
        let vals = vec![signal];
        let (state, signal_) = interpret(&states[i], &vals);
        states[i] = state;

        if let Some(s) = signal_ {
            signal = s;
        } else {
            break;
        }
        i = (i + 1) % states.len();
    }

    signal
}

pub fn part1() -> Num {
    let prog = &get_data_program(7);

    (0..5)
        .permutations(5)
        .map(|p| thruster_signal(&p, prog))
        .max()
        .unwrap()
}

pub fn part2() -> Num {
    let prog = &get_data_program(7);

    (5..10)
        .permutations(5)
        .map(|p| feedback_thruster_signal(&p, prog))
        .max()
        .unwrap()
}
