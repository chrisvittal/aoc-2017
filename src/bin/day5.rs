
extern crate aoc;

fn main() {
    let input: Vec<i32> = aoc::file::to_single_parsed("data/day5");
    println!("{}", skip(input.clone()));
    println!("{}", skip2(input));
}

fn skip(mut lst: Vec<i32>) -> u32 {
    let mut ret = 0;
    let mut ind: i32 = 0;
    while ind < lst.len() as i32 && ind >= 0 {
        let tmp = lst[ind as usize];
        lst[ind as usize] += 1;
        ind += tmp;
        ret += 1;
    }
    ret
}

fn skip2(mut lst: Vec<i32>) -> u32 {
    let mut ret = 0;
    let mut ind: i32 = 0;
    while ind < lst.len() as i32 && ind >= 0 {
        let tmp = lst[ind as usize];
        if tmp >= 3 {
            lst[ind as usize] -= 1;
        } else {
            lst[ind as usize] += 1;
        }
        ind += tmp;
        ret += 1;
    }
    ret
}
