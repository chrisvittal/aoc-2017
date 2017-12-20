
extern crate aoc;
extern crate regex;
#[macro_use]
extern crate lazy_static;
extern crate itertools;

use regex::Regex;
use std::str::FromStr;
use itertools::Itertools;

const INPUT: &'static str = "data/day20";
const RESTR: &'static str =
    r"p=<(-?\d+),(-?\d+),(-?\d+)>, v=<(-?\d+),(-?\d+),(-?\d+)>, a=<(-?\d+),(-?\d+),(-?\d+)>";
lazy_static! {
    static ref RE: Regex = Regex::new(RESTR).unwrap();
}

#[derive(Debug,PartialOrd,Ord,PartialEq,Eq,Copy,Clone)]
struct Point3D {
    x: isize,
    y: isize,
    z: isize,
}

#[derive(Debug,Eq,PartialEq,Copy,Clone)]
struct Particle {
    pos: Point3D,
    vel: Point3D,
    acc: Point3D,
}

impl std::ops::Add for Point3D {
    type Output = Self;
    fn add(self, rhs: Self) -> Self {
        Point3D {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}

impl Particle {
    fn step(self) -> Self {
        let vel = self.acc + self.vel;
        let pos = self.pos + vel;
        Particle { vel, pos, acc: self.acc }
    }
}

impl FromStr for Particle {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, ()> {
        let caps = RE.captures(s).ok_or(())?;
        let mut arr = [0; 9];

        for i in 0..9 {
            let x = caps.get(i+1).ok_or(())?.as_str().parse::<isize>().map_err(|_| ())?;
            arr[i] = x;
        }
        Ok(Particle {
            pos: Point3D { x: arr[0], y: arr[1], z: arr[2] },
            vel: Point3D { x: arr[3], y: arr[4], z: arr[5] },
            acc: Point3D { x: arr[6], y: arr[7], z: arr[8] },
        })
    }
}

fn main() {
    let mut input: Vec<Particle> = aoc::file::to_single_parsed(INPUT);

    {
        let max = input
        .iter().enumerate()
        .min_by_key(|&(_,&p)| (p.acc.x.abs() + p.acc.y.abs() + p.acc.z.abs(),
                               p.vel.x.abs() + p.vel.y.abs() + p.vel.z.abs(),
                               p.pos.x.abs() + p.pos.y.abs() + p.pos.z.abs()))
        .unwrap();
        println!("{:?}", max.0);
    }


    // Experimentation shows that 50 iterations is good enough.
    for _ in 0..50 {
        input.iter_mut().for_each(|p| *p = p.step());
        input.sort_by_key(|p| p.pos);
        let groups = input.into_iter().group_by(|p| p.pos);
        let mut new = Vec::new();
        for (_, mut g) in groups.into_iter() {
            let a = g.next();
            if g.next().is_none() {
                new.push(a.unwrap());
            }
        }
        input = new;
    }
    println!("{}", input.len());
}
