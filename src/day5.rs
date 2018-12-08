use std::fs::File;
use std::io::prelude::*;

// 97-65
fn test_polymer(c1 : char, c2 : char) -> bool {
    (c1 as u8)-32 == c2 as u8 || (c2 as u8)-32 == c1 as u8
}

pub fn entry() {
    let mut f = File::open("day5-3.txt").expect("file not found");
    let mut contents = String::new();
    f.read_to_string(&mut contents)
    .expect("something went wrong reading the file");

    let chars : Vec<char> = contents.chars().collect();

    let mut i1 = 0;
    let mut i2 = 1;
    let mut total = chars.len();
    loop {
        // TODO - we can't have 2 lowercase or 2 uppercase, so we have to check casing here
        if test_polymer(chars[i1], chars[i2]) {
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