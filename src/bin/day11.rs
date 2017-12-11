
extern crate aoc;

#[derive(PartialEq,Eq,Clone,Copy,Debug)]
enum HexDir {
    No, Ne, Se, So, Sw, Nw
}

impl std::str::FromStr for HexDir {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use HexDir::*;
        match s {
            "ne" => Ok(Ne), "n" => Ok(No), "nw" => Ok(Nw),
            "se" => Ok(Se), "s" => Ok(So), "sw" => Ok(Sw),
            _ => Err(())
        }
    }
}

#[derive(PartialEq,Eq,Clone,Copy,Debug)]
struct HexGrid {
    x: isize,
    y: isize,
}

impl HexGrid {
    fn new() -> Self { HexGrid { x: 0, y: 0} }
    
    fn step(mut self, dir: HexDir) -> Self {
        use HexDir::*;
        match dir {
            No => {self.x += 1             },
            So => {self.x -= 1             },
            Ne => {             self.y += 1},
            Sw => {             self.y -= 1},
            Nw => {self.x += 1; self.y -= 1},
            Se => {self.x -= 1; self.y += 1},
        }
        self
    }

    #[inline]
    fn dist(&self) -> usize {
        ((self.x.abs() + self.y.abs() + (self.x + self.y).abs()) / 2) as usize
    }
}

fn find_dist_and_max(steps: &[HexDir]) -> (usize, usize) {
    let (g, m) = steps.iter()
        .fold((HexGrid::new(), 0), |(g, m), &s| {
            let g = g.step(s);
            let n = g.dist();
            if n > m {
                (g, n)
            } else {
                (g, m)
            }
        });
    (g.dist(), m)
}

const INPUT: &'static str = "data/day11";

fn main() {
    let input: Vec<HexDir> = aoc::file::first_line(INPUT)
        .split(",").map(|x| x.parse().unwrap()).collect();
    let (dist, max) = find_dist_and_max(&input);
    println!("1: {}", dist);
    println!("2: {}", max);
}

