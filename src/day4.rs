use std::fs::File;
use std::io::prelude::*;
extern crate regex;

use self::regex::Regex;

struct time {
    year: i32;
    month: i32;
    day: i32;
    hour: i32;
    min: i32;
}

enum type {
    guard(i32),
    sleep(),
    wake(),
}

struct entry {
    time: Time;

}

pub fn entry() {
    let mut _f = File::open("day4.txt").expect("file not found");
    let mut _contents = String::new();
    _f.read_to_string(&mut _contents)
    .expect("something went wrong reading the file");

    lazy_static! {
        static ref RE: Regex = Regex::new(r"\[([\d]*)-([\d]*)-([\d]*) (\d\d):(\d\d)\] ([\w #]*)").unwrap();
    }
    for g in RE.captures_iter(&_contents) {
        println!("{}-{}-{} {}:{} - {}", &g[1], &g[2], &g[3], &g[4], &g[5], &g[6]);
    }
} 