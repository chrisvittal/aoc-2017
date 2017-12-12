
use std::ops::{Add,AddAssign};
use std::collections::HashMap;

const INPUT: u32 = 312051;

fn main() {
    println!("1: {}", solve_one(INPUT as i32));
    let n = SpiralSumIterator::new().find(|&n| n > INPUT).unwrap();
    println!("2: {}", n);
}

fn solve_one(input: i32) -> i32 {
    if input <= 1 { return 0; }

    let ring = (((input - 1) as f64).sqrt() / 2.).ceil() as i32;
    ring + ((input - (4 * ring * ring - 2 * ring + 1)) % ring).abs()
}

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
struct Point { x: i32, y: i32 }
type Pt = Point;

impl Point {
    const NEIGHBORS: [Pt; 8] =
        [Pt { x: -1, y:  1 }, Pt { x: 0, y:  1 }, Pt { x: 1, y:  1 },
         Pt { x: -1, y:  0 },                     Pt { x: 1, y:  0 },
         Pt { x: -1, y: -1 }, Pt { x: 0, y: -1 }, Pt { x: 1, y: -1 }];
    fn new(x: i32, y: i32) -> Self { Point { x, y } }
}

impl Add for Point {
    type Output = Point;
    #[inline]
    fn add(self, rhs: Self) -> Self {
        Point { x: self.x + rhs.x, y: self.y + rhs.y }
    }
}

impl<'a, 'b> Add<&'a Point> for &'b Point {
    type Output = Point;
    fn add(self, rhs: &'a Point) -> Point { *self + *rhs }
}

impl AddAssign for Point {
    fn add_assign(&mut self, other: Self) {
        *self = *self + other
    }
}

struct SpiralSumIterator {
    backing: HashMap<Point, u32>,
    dir: Point,
    loc: Point,
    lim: usize,
    step: usize,
    need_extra: bool,
}

impl SpiralSumIterator {
    fn new() -> Self {
        let mut s = SpiralSumIterator {
            backing: HashMap::with_capacity(100), // heuristic based on examining
                                                  // mine and other people's inputs
                                                  // this should never need to reallocate
            dir: Pt::new(1,0),
            loc: Pt::new(0,0),
            lim: 1,
            step: 0,
            need_extra: true,
        };
        s.backing.insert(s.loc, 1);
        s
    }
}

impl Iterator for SpiralSumIterator {
    type Item = u32;
    fn next(&mut self) -> Option<Self::Item> {
        self.loc += self.dir;
        self.step += 1;
        if self.step == self.lim {
            self.step = 0;
            self.need_extra = !self.need_extra;
            if self.need_extra {
                self.lim += 1;
            }
            self.dir = Pt { x: -self.dir.y, y: self.dir.x };
        }
        let sum = Pt::NEIGHBORS.iter().fold(0, |acc, &neighbor|
            match self.backing.get(&(self.loc + neighbor)) {
                Some(v) => acc + v,
                None => acc,
            });
        self.backing.insert(self.loc,sum);
        Some(sum)
    }
}
