use crate::lib::get_data_program;
use crate::lib::{interpret, Data, Num, Program, ProgramState};

use crate::lib::{display, s_display, to_map, upside_down};

use std::collections::HashMap;

use counter::Counter;
use num_complex::Complex;

type Position = Complex<Num>;
type Tile = Num;
type Tiles = HashMap<Position, Tile>;

fn run(p: &Program) -> Tiles {
    let mut prog = ProgramState::base(p);

    let mut tiles = Tiles::new();

    let mut start = false;

    loop {
        let input = vec![0];
        let (prog_, res) = interpret(prog, &input, true);
        prog = prog_;
        if res.is_none() {
            break;
        }
        let x = res.unwrap();

        let (prog_, res) = interpret(prog, &input, true);
        prog = prog_;
        let y = res.unwrap();

        let (prog_, res) = interpret(prog, &input, true);
        prog = prog_;
        let tile = res.unwrap();

        let p = Position::new(x, y);

        if p == Position::new(-1, 0) {

            println!("{:?}",tile);
            start = true;

  
        }


        if start {
             // clear screen
         //    print!("{}[2J", 27 as char); 

         //   display(&tiles);
            for (k,v) in tiles.iter() {
                if v == &4 || v == &3  {
                    println!("{:?}",(k, v) );

                }
            }


            for (k,v) in tiles.iter() {
                if v == &4  {
                    println!("{:?}",(k, v) );

                 //   print!("{}[2J", 27 as char); 

            display(&tiles);

                }
            }

            if p == Position::new(-1, 0) {

                println!("{:?}",tile);
    
      
            }


        }


        

        let seen = tiles.insert(p, tile);
    }

    tiles
}

pub fn part1() -> usize {
    let prog = get_data_program(13);

    let grid = run(&prog);
    display(&grid);
    grid.values().collect::<Counter<&Num>>()[&2]
}

pub fn part2() -> Num {
    let mut prog = get_data_program(13);

    prog[0] = 2;
    let grid = run(&prog);

    grid[&Position::new(-1, 0)]


  //  todo!();
}
