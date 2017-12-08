extern crate aoc;
extern crate regex;
extern crate petgraph;
extern crate itertools;
#[macro_use] extern crate lazy_static;

use std::collections::HashMap;

use itertools::Itertools;
use petgraph::algo::toposort;
use petgraph::prelude::*;
use regex::*;

lazy_static! {
    static ref RE: Regex = Regex::new(r"(\w+) \((\d+)\)").unwrap();
}

const INPUT: &'static str = "data/day7";

type MyGraph = Graph<Node, ()>;

struct Node {
    nm: String,
    wg: u32,
    tot: u32,
}

impl Node {
    /// Defaults to `to` and `wg` being equal.
    fn new(nm: String, wg: u32) -> Self {
        Node { nm, wg, tot: wg }
    }
}

fn main() {
    let input: Vec<_> = aoc::file::to_strings(INPUT);
    let graph = build_graph(input);
    let sorted = toposort(&graph, None).unwrap();
    println!("1: {}", graph[sorted[0]].nm);
    println!("2: {}", solve2(sorted, graph));
}

fn build_graph(v: Vec<String>) -> MyGraph {
    let v: Vec<_> = v.into_iter().map(parse).collect();
    let mut graph = Graph::new();
    let mut idx_map = HashMap::new();
    for &(ref par, val, _) in &v {
        let nd = graph.add_node(Node::new(par.to_string(), val));
        idx_map.insert(par.to_string(), nd);
    }

    for (par, _, chi) in v {
        for c in chi {
            graph.add_edge(idx_map[&par], idx_map[&c], ());
        }
    }
    graph
}

fn parse(s: String) -> (String, u32, Vec<String>) {
    let c: Vec<&str> = s.split(" -> ").collect();
    let c2 = c.get(1).map(|s| s.split(", ").map(|s| s.into()).collect());
    let cap = RE.captures(c[0]).unwrap();
    let nm = cap.get(1).unwrap().as_str().into();
    let wg = cap.get(2).unwrap().as_str().parse().unwrap();
    (nm, wg, c2.unwrap_or(vec![]))
}

fn solve2(sorted: Vec<NodeIndex>, mut graph: MyGraph) -> u32 {
    for &node in sorted.iter().rev() {
        if !graph.neighbors(node).map(|n| graph[n].tot).all_equal() {
            let (min, max) = graph.neighbors(node).map(|n| graph[n].tot)
                .minmax().into_option().unwrap();
            let (l, r): (Vec<_>, Vec<_>) = graph.neighbors(node).partition(|&n| graph[n].tot == min);
            let ub = if l.len() == 1 { &graph[l[0]] } else { &graph[r[0]] };
            return ub.wg + min - max;
        }

        graph[node].tot += graph.neighbors(node).map(|n| graph[n].tot).sum::<u32>();
    }
    panic!()
}
