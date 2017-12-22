
extern crate aoc;

use std::collections::HashMap;

const INPUT: &'static str = "data/day22";
type GridMap = HashMap<Point, NodeState>;

fn main() {
    let input = aoc::file::to_strings(INPUT);
    let w = input.len();
    let h = input[0].len();
    let start = Point { x: w as i32/2, y: h as i32/2};
    let map: GridMap = input.iter().enumerate()
        .flat_map(|(y,l)| {
            l.bytes().enumerate().filter_map(move |(x, b)| {
                if b == b'#' {
                    Some((Point { x: x as i32, y: y as i32}, NodeState::Infected))
                } else {
                    None
                }
            })
        }).collect();
    let mut v1 = Virus::new(map.clone(), start, false);
    println!("1: {}", v1.nth(10_000-1).unwrap());
    let mut v2 = Virus::new(map.clone(), start, true);
    println!("2: {}", v2.nth(10_000_000-1).unwrap());
}

struct Virus {
    map: GridMap,
    pos: Point,
    dir: Dir,
    inf: usize,
    pt2: bool
}

impl Virus {
    fn new(map: GridMap, pos: Point, pt2: bool) -> Self {
        Virus {
            map,
            pos,
            dir: Dir::Up,
            inf: 0,
            pt2,
        }
    }
}

impl Iterator for Virus {
    type Item = usize;
    fn next(&mut self) -> Option<Self::Item> {
        use NodeState::*;
        let ent = self.map.entry(self.pos).or_insert(Clean);
        match *ent {
            Clean => {
                self.dir = self.dir.left();
                *ent = if self.pt2 {
                    Weakend
                } else {
                    self.inf += 1;
                    Infected
                };
            },
            Infected => {
                self.dir = self.dir.right();
                *ent = if self.pt2 { Flagged } else { Clean }
            },
            Weakend => {
                *ent = Infected;
                self.inf += 1;
            },
            Flagged => {
                self.dir = self.dir.rev();
                *ent = Clean;
            },
        }
        self.pos += self.dir;
        Some(self.inf)
    }
}

#[derive(Clone,Copy,PartialEq,Eq,Debug)]
enum NodeState {
    Clean,
    Infected,
    Weakend,
    Flagged
}

#[derive(Clone,Copy,PartialEq,Eq,Hash,Debug)]
struct Point { x: i32, y: i32 }

impl ::std::ops::Add<Dir> for Point {
    type Output = Point;
    fn add(self, rhs: Dir) -> Self {
        use Dir::*;
        match rhs {
            Up => Point { x: self.x, y: self.y - 1},
            Rt => Point { x: self.x + 1, y: self.y},
            Dn => Point { x: self.x, y: self.y + 1},
            Lf => Point { x: self.x - 1, y: self.y},
        }
    }
}

impl ::std::ops::AddAssign<Dir> for Point {
    fn add_assign(&mut self, rhs: Dir) {
        *self = *self + rhs;
    }
}

#[derive(Clone,Copy,PartialEq,Eq,Debug)]
enum Dir {
    Up,
    Rt,
    Dn,
    Lf,
}

macro_rules! dir_impl{
    ($nm:ident, $up:ident, $rt:ident, $dn:ident, $lf:ident) =>
    {
        fn $nm(self) -> Self {
            use Dir::*;
            match self {
                Up => $up,
                Rt => $rt,
                Dn => $dn,
                Lf => $lf,
            }
        }
    }
}

impl Dir {
    dir_impl!(rev, Dn, Lf, Up, Rt);
    dir_impl!(left, Lf, Up, Rt, Dn);
    dir_impl!(right, Rt, Dn, Lf, Up);
}
