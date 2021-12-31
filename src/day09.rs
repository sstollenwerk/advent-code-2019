use crate::lib::get_data_program;
use crate::lib::{Data, Num, Program};

use itertools::Itertools;

#[derive(PartialEq, Clone, Eq, Debug)]
struct ProgramState {
    program: Program,
    pointer: usize,
    relative_base: Num,
}

impl ProgramState {
    fn new(program: &Program, pointer: usize, relative_base: Num) -> ProgramState {
        ProgramState {
            program: program.to_vec(),
            pointer,
            relative_base,
        }
    }

    fn base(program: &Program) -> ProgramState {
        ProgramState::new(program, 0, 0)
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

fn interpret(mut prog: ProgramState, input_: &Data) -> (ProgramState, Option<Num>) {
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

fn run(p: &Program, input: &Data) -> Data {
    let mut prog = ProgramState::base(p);

    let mut data = Vec::new();
    loop {
        let (prog_, res) = interpret(prog, input);
        prog = prog_;
        if let Some(n) = res {
            data.push(n)
        } else {
            break;
        }
    }
    data
}

pub fn part1() -> Num {
    let prog = get_data_program(9);

    let res = run(&prog, &vec![1]);
    println!("{:?}", res);

    res[0]

    //  todo!();
}

pub fn part2() -> Num {
    let prog = get_data_program(9);

    let res = run(&prog, &vec![2]);
    println!("{:?}", res);

    res[0]

}
