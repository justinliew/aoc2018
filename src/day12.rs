use std::fs::File;
use std::io::prelude::*;

pub fn entry() {
    let mut f = File::open("day12-2.txt").expect("file not found");
    let mut contents = String::new();
    f.read_to_string(&mut contents)
    .expect("something went wrong reading the file");

    let initial = "#..#.#..##......###...###";

    let lines = contents.split("\n");
    let mut rules = vec![];
    for line in lines {
        rules.push(line)
    }

    for r in rules {
        for c in initial.chars() {

        }
    }
}