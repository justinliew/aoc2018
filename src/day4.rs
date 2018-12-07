use std::fs::File;
use std::io::prelude::*;
use std::collections::BTreeMap;
use std::collections::HashMap;
extern crate regex;

use self::regex::Regex;

#[derive(Eq,Ord,PartialOrd,PartialEq,Debug)]
struct Time {
    year: i32,
    month: i32,
    day: i32,
    hour: i32,
    min: i32,
}

#[derive(Debug)]
enum EntryType {
    Guard(i32),
    Sleep(),
    Wake(),
}

fn check_overlaps(list: &Vec<Time>) {
    let mut minutes : [i32; 60] = [0; 60];

    let mut i = 0;
    while i < list.len()-1 {
        for t in list[i].min..list[i+1].min {
            print!("{} ", t);
            minutes[t as usize] += 1;
        }
        println!("");
        i += 2;
    }

    let mut i = 0;
    for m in minutes.iter() {
        println!("{} - {}", i, m);
        i += 1;
    }
}

pub fn entry() {
    let mut _f = File::open("day4.txt").expect("file not found");
    let mut _contents = String::new();
    _f.read_to_string(&mut _contents)
    .expect("something went wrong reading the file");

    lazy_static! {
        static ref RE: Regex = Regex::new(r"\[([\d]*)-([\d]*)-([\d]*) (\d\d):(\d\d)\] ([\w #]*)").unwrap();
    }

    //        let id = g[1].parse::<i32>().ok().unwrap();

    let mut events = BTreeMap::new();

    for g in RE.captures_iter(&_contents) {
        let t = Time {
            year: g[1].parse::<i32>().ok().unwrap(),
            month: g[2].parse::<i32>().ok().unwrap(),
            day: g[3].parse::<i32>().ok().unwrap(),
            hour: g[4].parse::<i32>().ok().unwrap(),
            min: g[5].parse::<i32>().ok().unwrap(),
        };
        if &g[6] == "falls asleep" {
            events.insert(t, EntryType::Sleep());
        } else if &g[6] == "wakes up" {
            events.insert(t, EntryType::Wake());
        } else {
            // Guard #2423 begins shift
            let splits = g[6].split_whitespace().collect::<Vec<&str>>();
            let num = splits[1].replace("#","").parse::<i32>().ok().unwrap();
            events.insert(t, EntryType::Guard(num));
        }
    }

    // for (k,v) in &events {
    //     println!("{:?} - {:?}",k, v);
    // }

    let mut guards = HashMap::new();

    let mut times = Vec::new();
    let mut guard_id = 0;
    for (k,v) in events {
        match v {
            EntryType::Guard(id) => {
                if guard_id > 0 {
                    let entry = guards.entry(guard_id).or_insert(vec![]);
                    entry.append(&mut times);
                }
                times.clear();
                guard_id = id;
            },
            EntryType::Sleep() => {
                times.push(k);
            },
            EntryType::Wake() => {
                times.push(k);
            }
        }
    }

    let mut max_guard_id = 0;
    let mut max_guard_time = 0;
    for (guard,list) in &guards {
        let mut guard_time = 0;
        let mut i = 0;
        if list.len () > 0 {
            while i < list.len()-1 {
                guard_time += list[i+1].min - list[i].min;
                i += 2;
            }
        }
        if guard_time > max_guard_time {
            max_guard_id = *guard;
            max_guard_time = guard_time;
        }
    }
    println!("Guard id {} TIme {}", max_guard_id, max_guard_time);

    match guards.get(&max_guard_id) {
        Some(list) => {
            check_overlaps(&list);
        },
        None => {},
    }
} 