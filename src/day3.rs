use std::fs::File;
use std::io::prelude::*;
extern crate regex;

pub fn entry() {
    let mut _f = File::open("day3.txt").expect("file not found");
    let mut _contents = String::new();
    _f.read_to_string(&mut _contents)
    .expect("something went wrong reading the file");

//    let mut _output : Vec<u8> = Vec::new();

    const MAX : usize = 1000;
    let mut m = [[0u8; MAX]; MAX];
    
    let _lines = _contents.split("\n");
    // #1 @ 249,597: 20x15
    for _line in _lines {
    }
}