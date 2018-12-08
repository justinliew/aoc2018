use std::fs::File;
use std::io::prelude::*;

// 97-65
fn test_polymer(c1 : char, c2 : char) -> bool {
    (c1 as u8)-32 == c2 as u8 || (c2 as u8)-32 == c1 as u8
}

pub fn entry() {
    let mut f = File::open("day5.txt").expect("file not found");
    let mut contents = String::new();
    f.read_to_string(&mut contents)
    .expect("something went wrong reading the file");

    let mut chars : Vec<char> = contents.chars().collect();

    let mut i1 = 0;
    let mut i2 = 1;
    let mut total = chars.len();
    loop {
        if test_polymer(chars[i1],chars[i2]) {
            chars.remove(i2);
            chars.remove(i1);
//            let s: String = chars.iter().collect();
//            println!("{} (removing {},{})", s,i1,i2);
            total -= 2;
            if i1 > 0 {
                i1 -=1;
            }
            if i2 > 1 {
                i2 -=1;
            }
        } else {
            i1 += 1;
            i2 += 1;
        }
        if i1 >= chars.len() || i2 >= chars.len () {
            break;
        }
    }

    println!("Total {}", total);
}