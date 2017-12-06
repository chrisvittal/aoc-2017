
extern crate aoc;

use std::collections::HashMap;

const FILENAME: &'static str = "data/day6";

fn main() {
    let input = aoc::file::to_split_parsed(FILENAME).pop().unwrap();
    let (p1, p2) = count_cycles(input);
    println!("1: {}\n2: {}", p1, p2);
}

/// Returns both the answer to part 1 and part 2
fn count_cycles(mut v: Vec<usize>) -> (usize, usize) {
    let mut seen = HashMap::new();
    loop {
        let tmp = seen.len();
        seen.insert(v.clone(), tmp);
        cycle(&mut v);
        if let Some(i) = seen.get(&v) {
            return (seen.len(), seen.len() - i);
        }
    }
}

/// Mutates in place as we clone the vector before doing this.
fn cycle(v: &mut Vec<usize>) {
    if let Some((i, &m)) = v.iter()
        .enumerate()
        .max_by_key(|&(i, x)| (x, -(i as isize))) {
        let l = v.len();
        v[i] = 0;
        for j in i+1..i+m+1 {
            v[j % l] += 1;
        }
    }
}

