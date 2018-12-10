use std::fs::File;
use std::io::prelude::*;
use std::collections::HashMap;
use std::collections::BTreeSet;
use std::collections::hash_map::Entry;

extern crate regex;

use self::regex::Regex;

pub fn entry() {
    let mut _f = File::open("day7.txt").expect("file not found");
    let mut _contents = String::new();
    _f.read_to_string(&mut _contents)
    .expect("something went wrong reading the file");

    lazy_static! {
        static ref RE: Regex = Regex::new(r"Step (\w) must be finished before step (\w) can begin.").unwrap();
    }

    let mut graph = HashMap::new();
    let mut depcount = HashMap::new();
    let mut total_set = BTreeSet::new();
    let mut non_roots = BTreeSet::new();
    for g in RE.captures_iter(&_contents) {
        let a = g[1].parse::<char>().ok().unwrap();
        let b = g[2].parse::<char>().ok().unwrap();
        total_set.insert(a);
        total_set.insert(b);
        non_roots.insert(b);
        let e = graph.entry(a).or_insert(vec![]);
        e.push(b);
        let d = depcount.entry(b).or_insert(0);
        *d += 1;
    }
    let mut frontier = total_set.difference(&non_roots).cloned().collect::<Vec<char>>();
    let mut order = vec![];

    println!("Frontier: {:?}", frontier);

    loop {
        frontier.sort_unstable_by(|a, b| b.cmp(a));
        frontier.dedup();
        let front = frontier.pop();
        if let Some(v) = front {
            let d = depcount.entry(v).or_insert(1);
            *d -= 1;
            println!("Testing push of  {} - {}", v, *d);
            if *d <= 0 {
                order.push(v);
                if let Entry::Occupied(mut o) = graph.entry(v) {
                    let mut copy = o.get_mut();
                    println!("Appending {:?}", copy);
                    frontier.append(copy);
                }
            } else {
                frontier.push(v);
            }
        }

        if frontier.is_empty() {
            break;
        }
    }

    for o in order {
        print!("{}",o);
    }
}