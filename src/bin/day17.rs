
const INPUT: usize = 312;
const LIMIT: usize = 50_000_000;

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
    let mut next = 0;
    let mut i = 1;
    while i <= LIMIT {
        next = (next + INPUT) % i + 1;
        if next == 1 { ans = i; }
        let fits = (i - next) / INPUT;
        next += fits * (INPUT + 1);
        i += fits + 1;
    }
    println!("2: {}", ans);
}
