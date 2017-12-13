
extern crate petgraph;

use petgraph::prelude::*;
use std::io::{Cursor,BufRead};

static INPUT: &'static str = include_str!("../../data/day12");

fn main() {
    let input = Cursor::new(INPUT).lines().map(|l| l.unwrap());
    let graph = parse_input(input);
    println!("1: {}", count(&graph, 0));
    println!("2: {}", petgraph::algo::connected_components(&graph));
}

fn count<T, U>(g: &UnGraphMap<T, U>, start: T) -> usize
where
    T: Copy + std::hash::Hash + Ord
{
    let mut bfs = Bfs::new(g, start);
    let mut count = 0;
    while let Some(_) = bfs.next(g) { count += 1; }
    count
}

fn parse_input<I: Iterator<Item = String>>(input: I) -> UnGraphMap <u32,()> {
    let mut graph = UnGraphMap::new();
    input.for_each(|line| {
        let mut it = line.split(" <-> ");
        let src = it.next().unwrap().parse().unwrap();
        let dests = it.next()
            .into_iter()
            .flat_map(|s| s.split(", ").map(|v| v.parse().unwrap()));
        graph.extend(dests.zip(std::iter::repeat(src)))
    });
    graph
}
