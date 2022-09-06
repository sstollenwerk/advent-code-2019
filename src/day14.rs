use crate::lib::{to_filename};
use std::fs;


use std::collections::HashMap;
use std::collections::HashSet;

type Num = u64;

type Key = (Num, String);

type Builds = Vec<Key>;

type Graph = HashMap<Key, Builds>;

fn get_data() -> Graph {
    let mut res = Graph::new();
    let mut seen = HashSet::new();

    for (xs, y) in fs::read_to_string(to_filename(14))
        .expect("Could not read file")
        .lines()
        .map(read_row)
    {

        assert!(   seen.insert(y.1.clone() )  );

        res.insert(y, xs);

    }

    res
}

fn read_row(row: &str) -> (Builds, Key) {
    let parts: Vec<_> = row.split(" => ").map(to_keys).collect();
    (parts[0].clone(), parts[1][0].clone())
}

fn to_keys(row: &str) -> Vec<Key> {
    let parts = row
        .split(',')
        .map(|s| s.trim().split(' ').collect::<Vec<_>>());
    parts.map(|p| (p[0].parse::<Num>().unwrap(), p[1].to_string())).collect()
}

fn topological_ordering(g_: &Graph) -> Vec<Key> {
    let mut g = g_.clone();
    let mut res = Vec::new();
    loop {
        let posses: HashSet<_> = g.keys().map(|s| s.1.clone()).collect();
        let remainder: HashSet<_> = g.values().flatten().map(|s| s.1.clone()).collect();
        let to_see = posses.difference(&remainder);
     if let Some(s) = to_see.next() {
        res.push(s);


     }
    
     else {break}
    
    }


}


pub fn part1() -> Num {
    let poses = get_data();
    println!("{:?}", poses);

    todo!();
}

pub fn part2() -> Num {
    let poses = get_data();
    println!("{:?}", poses);

    todo!();
}
