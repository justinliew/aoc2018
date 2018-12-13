use std::fs::File;
use std::io::prelude::*;
use std::collections::HashSet;

pub fn entry() {
    let mut f = File::open("day12.txt").expect("file not found");
    let mut contents = String::new();
    f.read_to_string(&mut contents)
    .expect("something went wrong reading the file");

    let lines = contents.split("\n");
    for line in lines {
    }
}