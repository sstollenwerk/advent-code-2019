use crate::lib::to_filename;

use std::fs;

use core::hash::Hash;
use core::hash::Hasher;

use std::collections::HashMap;

use cached::proc_macro::cached;

//use petgraph::Graph;
// &str is not a valid NodeIndex
// https://stackoverflow.com/questions/70330426/how-do-i-make-an-unweighted-undirected-graph-with-string-nodes-in-rust

type Num = u32;

#[derive(PartialEq, Clone, Eq, Debug)]
struct Graph(HashMap<String, String>);

impl Hash for Graph {
    fn hash<H: Hasher>(&self, state: &mut H) {
        1.hash(state);
    }
}

use std::ops::Deref;

impl Deref for Graph {
    type Target = HashMap<String, String>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

fn get_data() -> Graph {
    let mut parents = HashMap::new();

    for (obj, moon) in fs::read_to_string(to_filename(6))
        .expect("Could not read file")
        .lines()
        .map(read_row)
    {
        let k = parents.insert(moon, obj);
        assert!(k == None);
    }
    Graph(parents)
}

fn read_row(row: &str) -> (String, String) {
    let vals: Vec<_> = row.split(')').map(|s| s.to_string()).collect();
    (vals[0].clone(), vals[1].clone())
}

#[cached]
fn num_ancestors(s: String, g: Graph) -> Num {
    ancestors(s, g).len().try_into().unwrap()
}

#[cached]
fn ancestors(s: String, g: Graph) -> Vec<String> {
    if let Some(a) = g.get(&s) {
        let k = a.to_string();
        let mut prev = ancestors(k.clone(), g);
        prev.push(k);
        prev
    }

    else {
        Vec::new()
    }

}

fn orbits(g: Graph) -> Num {
    g.keys()
        .map(|s| num_ancestors(s.to_string(), g.clone()))
        .sum()
}



pub fn part1() -> Num {
    println!("{:?}", get_data());
    orbits(get_data())
}

pub fn part2() -> Num {
    todo!();
}
