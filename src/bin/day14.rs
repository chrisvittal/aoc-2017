
extern crate aoc;
extern crate bit_vec;

use bit_vec::BitVec;
use std::collections::{HashSet,VecDeque};

use aoc::knothash::KnotHash;
const INPUT: &'static str = "vbqugkhl";

fn main() {

    let mut coords = HashSet::new();

    let mut ans = 0;
    for i in 0..128 {
        let s = format!("{}-{}", INPUT, i);
        let hash = KnotHash::hash(s.as_ref()).dense().0;

        for b in hash.iter() {
            ans += b.count_ones();
        }

        let bits = BitVec::from_bytes(&hash);
        coords.extend(bits.into_iter().enumerate().filter_map(|(j,b)| {
            if b { Some((j as i32, i as i32)) } else { None }
        }));
    }
    println!("1: {}", ans);
    
    println!("2: {}", cc(coords));
}

fn cc(mut set: HashSet<(i32,i32)>) -> u32 {
    let mut cc = 0;
    let mut queue = VecDeque::new();
    loop
    {
        let k = if let Some(k) = set.iter().next() {
            *k
        } else {
            return cc;
        };

        set.remove(&k);

        cc+=1;
        queue.push_back(k);
        while let Some((x, y)) = queue.pop_front() {
            queue.extend(set.take(&(x - 1, y)));
            queue.extend(set.take(&(x, y - 1)));
            queue.extend(set.take(&(x + 1, y)));
            queue.extend(set.take(&(x, y + 1)));
        }
    }
}

