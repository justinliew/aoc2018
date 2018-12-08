use std::fs::File;
use std::io::prelude::*;

pub fn entry() {
    let mut f = File::open("day5-2.txt").expect("file not found");
    let mut contents = String::new();
    f.read_to_string(&mut contents)
    .expect("something went wrong reading the file");

    contents = contents.to_lowercase();
    let chars : Vec<char> = contents.chars().collect();

    let mut i1 = 0;
    let mut i2 = 1;
    let mut total = chars.len();
    loop {
        if chars[i1] == chars[i2] {
            i1 -= 1;
            i2 += 1;
            total -= 2;
        } else {
            if i2 - i1 > 1 {
                i2 += 2;
                i1 = i2-1;
            } else {
                i1 += 2;
                i2 += 2;
            }
        }
        if i1 >= chars.len() || i2 >= chars.len() {
            break;
        }
    }
    println!("Total {}", total);
}