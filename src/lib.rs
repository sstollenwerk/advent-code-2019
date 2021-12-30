use std::fs;

pub fn to_filename(day: i32) -> String {
    format!("input/{:0>2}.txt", day)
}
