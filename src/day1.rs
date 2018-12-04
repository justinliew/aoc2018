use std::fs::File;
use std::io::prelude::*;
use std::collections::HashSet;

pub fn entry() {
    let mut _f = File::open("day1.txt").expect("file not found");
    let mut _contents = String::new();
    _f.read_to_string(&mut _contents)
    .expect("something went wrong reading the file");

    let mut total = 0;
    let mut history = HashSet::new();
    loop {
        let _lines = _contents.split("\n");
        for _line in _lines {
            let (sign,numstring) = _line.split_at(1);
            let num : i32 = numstring.trim().parse().ok().unwrap();
            match sign {
                "+" => total = total + num,
                "-" => total = total - num,
                _ => continue
            }
            if history.contains(&total) {
                println!("We found this again: {}", total);
                return
            }
            history.insert(total);
        }
    }
    println!("total: {}", total);
}