use std::fs::File;
use std::io::prelude::*;
extern crate regex;

use self::regex::Regex;

pub fn entry() {
    let mut _f = File::open("day3.txt").expect("file not found");
    let mut _contents = String::new();
    _f.read_to_string(&mut _contents)
    .expect("something went wrong reading the file");

//    let mut _output : Vec<u8> = Vec::new();

    const MAX : usize = 1000;
    let mut m = [[0i32; MAX]; MAX];
    
    let _lines = _contents.split("\n");
    // #1 @ 249,597: 20x15

    for _line in _lines {
        let local_line = _line;
//        println!("Line: {}", _line);
        lazy_static! {
            static ref RE: Regex = Regex::new(r"^#[\d]* @ ([\d]*),([\d]*): ([\d]*)x([\d]*)$").unwrap();
        }
        for g in RE.captures_iter(local_line) {
            let x = g[1].parse::<usize>().ok().unwrap();
            let y = g[2].parse::<usize>().ok().unwrap();
            let w = g[3].parse::<i32>().ok().unwrap();
            let h = g[4].parse::<i32>().ok().unwrap();
            m[x][y] = w;
            println!("m[{}][{}] = {}", x,y,w);
//            m[g[1]][g[2]] = g[3];
        }
    }
    println!("m[0][0] = {}", m[249][597]);
}