
extern crate aoc;

fn main() {
    let s = aoc::file::to_split_parsed("data/day04");
    let x = s.iter().filter(|v| is_valid_base(v)).count();
    println!("1: {}", x);
    let y = s.iter().filter(|v| is_valid2(v)).count();
    println!("2: {}", y);
}

fn is_valid2(strs: &[String]) -> bool {
    let new: Vec<Vec<char>> = strs.iter().map(|s| {
        let mut x = s.chars().collect::<Vec<char>>();
        x.sort();
        x
    }).collect();
    is_valid_base(&new)
}

fn is_valid_base<T: Eq>(vals: &[T]) -> bool {
    for (i, s) in vals.iter().enumerate() {
        if i+1 == vals.len() { break; }
        for t in vals[i+1..].iter() {
            if s == t { return false; }
        }
    }
    true
}
