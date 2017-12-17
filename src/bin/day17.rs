
const INPUT: usize = 312;

fn main() {
    let mut v = vec![0];
    let mut next = 0;
    for i in 1..2018 {
        next = 1 + (next+INPUT) % v.len();
        v.insert(next, i);
    }
    let p = v.iter().position(|x| *x == 2017).unwrap();
    println!("1: {}", v[p+1]);

    let mut ans = 0;
    next = 0;
    for i in 1..50_000_001 {
        next = (next + INPUT) % i;
        if next == 0 { ans = i; }
        next += 1;
    }
    println!("2: {}", ans);
}
