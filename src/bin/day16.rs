
#![feature(slice_rotate)]
extern crate aoc;

use std::str::FromStr;

const INPUT: &'static str = "data/day16";

enum Dance {
    Spin(u16),
    Exchange(u8,u8),
    Partner(u8,u8),
}

impl FromStr for Dance {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use Dance::*;
        let bs = s.as_bytes();
        let ss = &s[1..];
        match bs[0] {
            b's' => match ss.parse::<u16>() {
                Ok(v) => Ok(Spin(v)),
                Err(_) => Err(()),
            },
            b'x' => {
                let mut it = ss.split("/");
                let a = it.next().unwrap().parse().unwrap();
                let b = it.next().unwrap().parse().unwrap();
                Ok(Exchange(a,b))
            }
            b'p' => {
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
            prgs.rotate(t - (v as usize % t));
        },
        Exchange(a, b) => prgs.swap(a as usize, b as usize),
        Partner(a, b) => {
            let a = prgs.iter().position(|&v| v == a).unwrap();
            let b = prgs.iter().position(|&v| v == b).unwrap();
            prgs.swap(a,b);
        }
    }
    prgs
}

fn find(init: [u8; 16], dnc: &[Dance], f: fn([u8; 16], &[Dance]) -> [u8; 16],
         n: usize) ->  [u8; 16] {
    let mut tortoise = f(init, dnc);
    let mut hare = f(f(init, dnc), dnc);
    let mut v = vec![init, tortoise];
    while (tortoise != hare) {
        tortoise = f(tortoise, dnc);
        v.push(tortoise);
        hare = f(f(hare, dnc), dnc);
    }

    let mut mu = 0;
    tortoise = init;
    while (tortoise != hare) {
        tortoise = f(tortoise, dnc);
        hare = f(hare, dnc);
        mu += 1;
    }

    let mut lam = 1;
    hare = f(hare, dnc);
    while (tortoise != hare) {
        hare = f(hare, dnc);
        v.push(hare);
        lam += 1;
    }
    v[(n - mu) % lam]
}

fn main() {
    let input: Vec<Dance> = aoc::file::first_line(INPUT).split(",").map(|v| v.parse().unwrap()).collect();
    let ans = input.iter().fold(START, |prgs, dnc| dance(prgs, dnc));
    println!("{}", ans.iter().map(|b| *b as char).collect::<String>());
    let ans = find(START, &input, |prgs, dncs| dncs.iter().fold(prgs, |p, d| dance(p, d)),
                    1_000_000_000);
    println!("{}", ans.iter().map(|b| *b as char).collect::<String>());
}

