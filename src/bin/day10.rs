
extern crate aoc;

use aoc::knothash::{self, KnotHash};

const INPUT: &'static str = "data/day10";

fn main() {
    let input = aoc::file::first_line(INPUT);
    let input_num: Vec<usize> = input.trim().split(",")
        .map(|b| b.parse().unwrap()).collect();
    println!("1: {}", solve1(&input_num));
    let lengths = input.trim().as_bytes();
    println!("2: {}", KnotHash::hash(&lengths));
}

fn solve1(input: &[usize]) -> usize {
    let mut pos = 0;
    let mut skip = 0;
    let mut lst = KnotHash::START;
    for &l in input {
        knothash::rev(&mut lst, pos, l);
        pos += l + skip;
        skip += 1;
    }
    lst[0] as usize * lst[1] as usize
}
