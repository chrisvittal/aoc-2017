
extern crate aoc;
extern crate petgraph;

use petgraph::prelude::*;
    
use std::collections::HashMap;

const INPUT: &'static str = "data/day12";

fn main() {
    let input = aoc::file::to_lines(INPUT).map(|l| l.unwrap());
    let graph = parse_input(input);
    println!("{}", petgraph::algo::connected_components(&graph));
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
/*
fn build_graph<I: Iterator<Item = (u32, Vec<u32>)>>(iter: I) -> UnGraphMap<u32,()> {
    let mut graph = UnGraphMap::new();
    for (src, dests) in iter {
        let n = if let Some(&n) = nodemap.get(&src) {
            n
        } else {
            let n = graph.add_node(src);
            nodemap.insert(src, n);
            n
        };
        
        for d in dests {
            let m = if let Some(&n) = nodemap.get(&d) {
                n
            } else {
                let n = graph.add_node(d);
                nodemap.insert(d, n);
                n
            };
            graph.add_edge(n,m,());
        }
    }
    graph
}
*/
