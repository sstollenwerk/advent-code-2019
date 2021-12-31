use crate::lib::get_data_program;
use crate::lib::{Data, Num, Program};

fn required_positions(n: Num) -> Num {
    match n {
        1 => 3,
        2 => 3,
        3 => 1,
        4 => 1,
        99 => 0,
        _ => panic!(),
    }
}

fn interpret(mut data: Program, input_: &Data) -> Data {
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
        i += 1;

        match op {
            1 => *res = registers[0] + registers[1],
            2 => *res = registers[0] * registers[1],
            3 => *res = *input.next().unwrap(),
            4 => output.push(*res),
            _ => panic!(),
        }
    }

    output
}

pub fn part1() -> Num {
    let vals = get_data_program(5);

    let mut res = interpret(vals, &vec![1]);

    println!("{:?}", res);

    res.pop().unwrap()
}

pub fn part2() -> Num {
    todo!();
}
