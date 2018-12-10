use std::fs::File;
use std::io::prelude::*;
use std::collections::HashMap;
use std::collections::HashSet;

fn manhattan(lhs : (i32,i32), rhs : (i32,i32)) -> i32 {
    (lhs.0-rhs.0).abs() + (lhs.1-rhs.1).abs()
}

fn closest(points : &Vec<(i32,i32)>, grid : (i32,i32)) -> i32 {
    let mut index = 0;
    let mut closest_value = 99999;
    let mut closest_index = 0;
    let mut closest_count = 0;
    for pt in points.iter() {
        let value = manhattan(*pt,grid);
        if value == closest_value {
            closest_count += 1;
        }
        if value < closest_value {
            closest_value = value;
            closest_index = index;
            closest_count = 0;
        }
        index += 1;
    }
    if closest_count > 0 {
        return -1
    }
    closest_index
}

fn man_sum(points : &Vec<(i32,i32)>, grid : (i32,i32)) -> i32 {
    let mut total = 0;
    for pt in points.iter() {
        total += manhattan(*pt,grid);
    }
    total
}

pub fn entry() {
    let mut f = File::open("day6.txt").expect("file not found");
    let mut contents = String::new();
    f.read_to_string(&mut contents)
    .expect("something went wrong reading the file");

    let mut min_x = 99999;
    let mut min_y = 99999;
    let mut max_x = 0;
    let mut max_y = 0;

    let lines = contents.split("\n");
    let mut coords = vec![];
    for line in lines {
        let elems = line.split(",").collect::<Vec<&str>>();
        let x = elems[0].trim().parse::<i32>().ok().unwrap();
        let y = elems[1].trim().parse::<i32>().ok().unwrap();
        if x > max_x {
            max_x = x;
        }
        if x < min_x {
            min_x = x;
        }
        if y > max_y {
            max_y = y;
        }
        if y < min_y {
            min_y = y;
        }

        let pair = (x,y);
        coords.push(pair);
    }

    let mut counts = HashMap::new();
    let mut infinity = HashSet::new();
    println!("Min ({},{}), Max ({},{})", min_x, min_y, max_x, max_y);
    for w in min_x..max_x+1 {
        for h in min_y..max_y+1 {
            let index = closest(&coords,(w,h));
            if index == -1 {
                continue
            }
            let c = counts.entry(index).or_insert(0);
            *c += 1;
            if w == min_x || w == max_x || h == min_y || h == max_y {
                infinity.insert(index);
            }
        }
    }

    println!("Infinity {:?}", infinity);
    println!("Counts {:?}", counts);
    let mut total = 0;
    for (k,v) in counts.iter() {
        if *k != -1 {
            total += v;
        }
    }
    println!("Total {} vs {}", total, (max_x-min_x)*(max_y-min_y));

    let mut max_count = 0;
    let mut max_index = 0;
    for (k,v) in counts.iter() {
        if *k == -1 {
            continue;
        }
        if !infinity.contains(k) {
            if v > &max_count {
                max_count = *v;
                max_index = *k;
            }
        }
    }    
    println!("Max Count: {} {}", max_count, max_index);
}

pub fn entry2() {
    let mut f = File::open("day6.txt").expect("file not found");
    let mut contents = String::new();
    f.read_to_string(&mut contents)
    .expect("something went wrong reading the file");

    let mut min_x = 99999;
    let mut min_y = 99999;
    let mut max_x = 0;
    let mut max_y = 0;

    let lines = contents.split("\n");
    let mut coords = vec![];
    for line in lines {
        let elems = line.split(",").collect::<Vec<&str>>();
        let x = elems[0].trim().parse::<i32>().ok().unwrap();
        let y = elems[1].trim().parse::<i32>().ok().unwrap();
        if x > max_x {
            max_x = x;
        }
        if x < min_x {
            min_x = x;
        }
        if y > max_y {
            max_y = y;
        }
        if y < min_y {
            min_y = y;
        }

        let pair = (x,y);
        coords.push(pair);
    }

    let mut num = 0;
    for w in min_x..max_x+1 {
        for h in min_y..max_y+1 {
            let sum = man_sum(&coords,(w,h));
            if sum < 10000 {
                num += 1;
            }
        }
    }

    println!("Region {}", num);
}