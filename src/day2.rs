use std::fs::File;
use std::io::prelude::*;
use std::collections::HashMap;

pub fn entry() {
    let mut _f = File::open("day2.txt").expect("file not found");
    let mut _contents = String::new();
    _f.read_to_string(&mut _contents)
    .expect("something went wrong reading the file");

    let _lines = _contents.split("\n");
    let mut _doubles = 0;
    let mut _triples = 0;
    for _line in _lines {
        println!("Line - {}", _line);
        let _bytes = _line.as_bytes();
        let mut _map = HashMap::new();
        for _b in _bytes {
            let _entry = _map.entry(_b).or_insert(0);
            *_entry += 1;
        }
        let mut _found2 = false;
        let mut _found3 = false;
        for (byte,count) in &_map {
            if *count == 2 && !_found2 {
                _doubles += 1;
                _found2 = true;
            }
            if *count == 3 && !_found3 {
                _triples += 1;
                _found3 = true;
            }
        }
        println!("{} + {}", _doubles, _triples);
    }
    println!("{}", _doubles * _triples); 
}

fn do_compare(lhs : &str, rhs : &str) -> bool {
    let lhs_bytes = lhs.as_bytes();
    let rhs_bytes = rhs.as_bytes();
    let mut found_one : bool = false;
    let mut found_pos : i32 = -1;
    for i in 0..lhs_bytes.len() {
        if lhs_bytes[i] != rhs_bytes[i] {
            if found_one {
                return false
            }
            found_one = true;
            found_pos = i as i32;
        }
    }
    if found_one {
        println!("{}", lhs);
        println!("{}", rhs);
        return true
    }
    false
}

pub fn entry2() {
    let mut _f = File::open("day2.txt").expect("file not found");
    let mut _contents = String::new();
    _f.read_to_string(&mut _contents)
    .expect("something went wrong reading the file");

    let _lines = _contents.split("\n");
    for _line in _lines {
        let _lines2 = _contents.split("\n");
        let mut found = false;
        for _line2 in _lines2 {
            found = do_compare(_line,_line2);
            if found {
                return
            }
        }
    }
}