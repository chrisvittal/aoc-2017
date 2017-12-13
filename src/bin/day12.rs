
extern crate aoc;

use std::collections::HashSet;
use aoc::unionfind::*;

static INPUT: &'static str = "data/day12";

fn main() {
    let input = aoc::file::to_lines(INPUT).map(|l| l.unwrap());
    let mut uf = parse_input(input);
    let mut cc = HashSet::new();
    let mut siz = 0;
    for i in 0..uf.len() {
        cc.insert(uf.find(i));
        if uf.find(i) == uf.find(0) {
            siz += 1;
        }
    }
    println!("1: {}", siz);
    println!("2: {}", cc.len());
}

fn parse_input<I: Iterator<Item = String>>(iter: I) -> UnionFind {
    let edges: Vec<(_, Vec<_>)> = iter.map(|line| {
        let mut it = line.split(" <-> ");
        let src = it.next().unwrap().parse().unwrap();
        let dests = it.next()
            .into_iter()
            .flat_map(|s| s.split(", ").map(|v| v.parse().unwrap()));
        (src, dests.collect())
    }).collect();
    let mut uf = UnionFind::new(edges.len());
    for (s, dests) in edges {
        for d in dests {
            uf.union(s, d);
        }
    }
    uf
}
