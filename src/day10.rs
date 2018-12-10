use std::fs::File;
use std::io::prelude::*;
extern crate regex;

use self::regex::Regex;

extern crate bmp;
use self::bmp::{Image, Pixel};

pub fn entry() {
    let mut f = File::open("day10.txt").expect("file not found");
    let mut contents = String::new();
    f.read_to_string(&mut contents)
    .expect("something went wrong reading the file");

    lazy_static! {
        static ref RE: Regex = Regex::new(r"position=<\s*([\d-]*),\s*([\d-]*)> velocity=<\s*([\d-]*),\s*([\d-]*)>").unwrap();
    }

    let mut coords = vec![];

    let mut min_x : i32 = 9999999;
    let mut min_y : i32 = 9999999;
    let mut max_x : i32 = 0;
    let mut max_y : i32 = 0;

    for g in RE.captures_iter(&contents) {
        let px = g[1].parse::<i32>().ok().unwrap();
        let py = g[2].parse::<i32>().ok().unwrap();
        let vx = g[3].parse::<i32>().ok().unwrap();
        let vy = g[4].parse::<i32>().ok().unwrap();
        let c = (px,py,vx,vy);

        if px > max_x {
            max_x = px;
        }
        if px < min_x {
            min_x = px;
        }
        if py > max_y {
            max_y = py;
        }
        if py < min_y {
            min_y = py;
        }
        coords.push(c);
    }

    let mut index = 0;
    loop {
        let width = max_x-min_x;
        let height = max_y-min_y;
        let coords = coords.iter_mut().map(|c| {c.0 += c.2;c.1 += c.3;c}).collect::<Vec<_>>();
        min_x = 999999;
        min_y = 999999;
        max_x = 0;
        max_y = 0;
        for c in &coords {
            if c.0 > max_x {
                max_x = c.0;
            }
            if c.0 < min_x {
                min_x = c.0;
            }
            if c.1 > max_y {
                max_y = c.1;
            }
            if c.1 < min_y {
                min_y = c.1;
            }
        }

        if width < 500 {
            println!("Writing image of size {}x{}", width, height);
            let mut img = Image::new(width as u32,height as u32);
            for c in &coords {
                img.set_pixel((c.0-min_x) as u32,(c.1-min_y) as u32, Pixel::new(255,255,255));
            }
            let mut w = Vec::new();
            write!(&mut w, "{}.bmp", index);
            let s : String = w.into_iter().map(|x| x as char).collect();
            let _ = img.save(s);
        }
        index += 1;
    }

}