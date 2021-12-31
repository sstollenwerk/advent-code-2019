use std::fs;

pub type Num = i32;
pub type Program = Vec<Num>;
pub type Data = Vec<Num>;

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
