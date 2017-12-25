
#![feature(slice_rotate)]
extern crate aoc;

use std::str::FromStr;

const INPUT: &'static str = "data/day16";

#[derive(Clone, Copy, PartialEq, Eq)]
enum Dance {
    Spin(usize),
    Exchange(usize,usize),
    Partner(u8,u8),
}

impl FromStr for Dance {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use Dance::*;
        let ss = &s[1..];
        match s {
            _ if s.starts_with("s")  => match ss.parse() {
                Ok(v) => Ok(Spin(v)),
                Err(_) => Err(()),
            },
            _ if s.starts_with("x") => {
                let mut it = ss.split("/");
                let a = it.next().unwrap().parse().unwrap();
                let b = it.next().unwrap().parse().unwrap();
                Ok(Exchange(a,b))
            }
            _ if s.starts_with("p") => {
                let mut it = ss.split("/");
                let a = it.next().unwrap().chars().nth(0).unwrap() as u8;
                let b = it.next().unwrap().chars().nth(0).unwrap() as u8;
                Ok(Partner(a,b))
            },
            _ => Err(())
        }
    }
}

const START: [u8; 16] = *b"abcdefghijklmnop";

fn dance(mut prgs: [u8; 16], mv: &Dance) -> [u8; 16] {
    use Dance::*;
    match *mv {
        Spin(v) => {
            let t = prgs.len();
            prgs.rotate(t - (v % t));
        },
        Exchange(a, b) => prgs.swap(a, b),
        Partner(a, b) => {
            let a = prgs.iter().position(|&v| v == a).unwrap();
            let b = prgs.iter().position(|&v| v == b).unwrap();
            prgs.swap(a,b);
        }
    }
    prgs
}

fn main() {
    let input: Vec<Dance> = aoc::file::first_line(INPUT).split(",").map(|v| v.parse().unwrap()).collect();
    let ans = input.iter().fold(START, |prgs, dnc| dance(prgs, dnc));
    println!("1: {}", ::std::str::from_utf8(&ans).unwrap());
    let ans = find(START, &input, |prgs, dncs| dncs.iter().fold(prgs, |p, d| dance(p, d)),
                   1_000_000_000);
    println!("2: {}", ::std::str::from_utf8(&ans).unwrap());
}

fn find(mut init: [u8; 16],
        dnc: &[Dance],
        f: fn([u8; 16], &[Dance]) -> [u8; 16],
        n: usize) -> [u8; 16] {
    let mut seen = ::std::collections::HashMap::new();
    let mut idx_map = ::std::collections::HashMap::new();
    for i in 0..n {
        if let Some(seen_id) = seen.get(&init) {
            let cycle = i - seen_id;
            let ind = (n - i) % cycle;
            return *idx_map.get(&ind).expect("missing element");
        }

        seen.insert(init, i);
        idx_map.insert(i, init);

        init = f(init, dnc);
    }
    unreachable!()
}
