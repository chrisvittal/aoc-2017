
extern crate aoc;

const INPUT: &'static str = "data/day13";

fn main() {
    let mut input: Vec<(usize,usize)> = aoc::file::to_lines(INPUT).map(|l| {
        let tmp = l.unwrap();
        let mut it = tmp.split(": ");
        let x: usize = it.next().unwrap().parse().unwrap();
        let y: usize = it.next().unwrap().parse().unwrap();
        (x, 2*(y-1))
    }).collect();
    input.sort_unstable_by_key(|&(x,y)| (y,x));
    let one = input.iter().fold(0, |a, &(d,r)| if d%r == 0 { a + d*(r+2)/2 } else { a });
    println!("1: {}", one);
    let two = (0..).find(|s| !input.iter().any(|&(d, r)| (d+s) % r == 0)).unwrap();
    println!("2: {}", two);
}
