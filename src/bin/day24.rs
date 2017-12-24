
extern crate aoc;

use std::str::FromStr;
use std::collections::HashSet;

#[derive(Debug,Clone,Copy)]
struct Component(u32,u32);

struct Answer {
    pt1: u32,
    pt2: (usize, u32)
}

impl FromStr for Component {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, ()> {
        let mut s = s.split("/").map(|d| d.parse().unwrap());
        let a = s.next().unwrap();
        let b = s.next().unwrap();
        Ok(Component(a,b))
    }
}

fn build(comps: &[Component],
         mut used: &mut HashSet<usize>,
         prev: u32) -> Answer {
    let mut max = Answer { pt1: 0, pt2: (0,0) };
    if comps.len() == used.len() { return max; }
    for (i,c) in comps.iter().enumerate() {
        if (c.0 == prev || c.1 == prev) && !used.contains(&i) {
            used.insert(i);
            let Answer { pt1, pt2 }= build(comps, &mut used, c.0 + c.1 - prev);
            let pt1 = pt1 + c.0 + c.1;
            let pt2 = (pt2.0 + 1, pt2.1 + c.0 + c.1);
            used.remove(&i);
            if pt1 > max.pt1 {
                max.pt1 = pt1
            }
            if pt2 > max.pt2 {
                max.pt2 = pt2;
            }
        }
    }
    max
}

const INPUT: &'static str = "data/day24";

fn main() {
    let input: Vec<Component> = aoc::file::to_single_parsed(INPUT);
    let mut set = HashSet::new();
    let Answer { pt1, pt2 } = build(&input, &mut set, 0);
    println!("1: {}", pt1);
    println!("2: {}", pt2.1);
}
