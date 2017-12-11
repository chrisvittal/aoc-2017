
extern crate aoc;

use aoc::file;

fn main() {
    let s = file::to_split_parsed("data/day02");
    println!("1: {}", checksum(&s));
    println!("2: {}", checksum2(&s));
}

fn checksum(data: &Vec<Vec<u32>>) -> u32 {
    let mut tot = 0;
    for v in data {
        tot += v.iter().max().unwrap() - v.iter().min().unwrap()
    }
    tot
}

fn checksum2(data: &Vec<Vec<u32>>) -> u32 {
    let mut tot = 0;
    for v in data {
        for (i, &d) in v.iter().enumerate() {
            for &d2 in v[i+1..].iter() {
               if d > d2 && d % d2 == 0 {
                   tot += d / d2;
               } else if d2 % d == 0 {
                   tot += d2 / d;
               }
            }
        }
    }
    tot
}
