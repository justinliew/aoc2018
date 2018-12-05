use std::fs::File;
use std::io::prelude::*;
use std::collections::HashMap;
extern crate regex;

use self::regex::Regex;

pub fn entry() {
    let mut _f = File::open("day3.txt").expect("file not found");
    let mut _contents = String::new();
    _f.read_to_string(&mut _contents)
    .expect("something went wrong reading the file");

    const MAX : usize = 1400;
    const DEPTH : usize = 1;
    // I need this to be a list of all overlaps
    let mut m = [[0i32; MAX]; MAX];
    let mut l = HashMap::new();

    lazy_static! {
        static ref RE: Regex = Regex::new(r"#([\d]*) @ ([\d]*),([\d]*): ([\d]*)x([\d]*)").unwrap();
    }
    for g in RE.captures_iter(&_contents) {
        let id = g[1].parse::<i32>().ok().unwrap();
        let x = g[2].parse::<usize>().ok().unwrap();
        let y = g[3].parse::<usize>().ok().unwrap();
        let width = g[4].parse::<usize>().ok().unwrap();
        let height = g[5].parse::<usize>().ok().unwrap();
        for w in 0..width {
            for h in 0..height {
                if m[x+w as usize][y+h as usize] == 0 {
                    m[x+w as usize][y+h as usize] = id;
                }
            }
        }
    }

    for (k,v) in &l {
        println!("{} {}", k,v);
    }
}