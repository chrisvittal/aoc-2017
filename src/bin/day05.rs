
extern crate aoc;

fn main() {
    let input: Vec<i32> = aoc::file::to_single_parsed("data/day05");
    println!("1: {}", skip(input.clone(), |_| 1));
    println!("2: {}", skip(input, |t| if t >= 3 { -1 } else { 1 }));
}

fn skip(mut lst: Vec<i32>, f: fn(i32) -> i32) -> u32 {
    let mut ret = 0;
    let mut ind = 0;
    while ind < lst.len() as i32 && ind >= 0 {
        let tmp = lst[ind as usize];
        lst[ind as usize] += f(tmp);
        ind += tmp;
        ret += 1;
    }
    ret
}
