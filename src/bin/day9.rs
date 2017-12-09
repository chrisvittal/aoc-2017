
extern crate aoc;

const INPUT: &'static str = "data/day9";

fn main() {
    let input = aoc::file::first_line(INPUT);
    let (score, cncld) = solve(&input);
    println!("1: {}\n2: {}", score, cncld);
}

/// First item in tuple is score, second is garbage characters
fn solve(s: &str) -> (u32, u32) {
    let mut chrs = s.chars();
    let mut garbage = false;
    let mut lvl = 0;
    let mut score = 0;
    let mut cncld = 0;

    while let Some(c) = chrs.next() {
        match c {
            '!' => {chrs.next();},
            '>' => garbage = false,
            _ if garbage => cncld += 1,
            '<' => garbage = true,
            '{' => lvl += 1,
            '}' => {score += lvl; lvl -= 1;},
            _ => {}
        }
    }
    (score, cncld)
}
