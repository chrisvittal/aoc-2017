
extern crate aoc;

fn main() {
    let input = aoc::file::first_line("data/day1");
    let ans = solve_both(&input);
    println!("1: {}\n2: {}", ans.0, ans.1);
}

fn solve_both(s: &str) -> (u32, u32) {
    let bytes = s.as_bytes().iter().map(|&b| b - '0' as u8).collect::<Vec<_>>();
    let len = bytes.len();

    bytes.iter()
        .enumerate()
        .fold((0,0), |(mut acc1, mut acc2), (i, &b)| {
            if b == bytes[(i+1) % len] {
                acc1 += b as u32;
            }
            if b == bytes[(i + len/2) % len] {
                acc2 += b as u32;
            }
            (acc1, acc2)
        })
}
