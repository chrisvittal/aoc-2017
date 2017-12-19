
const INPUT: &'static str = include_str!("../../data/day19");

fn main() {
    let input: Vec<Vec<u8>> = INPUT.lines().map(|l| l.bytes().collect()).collect();
    let x = input[0].iter().position(|&b| b == b'|').unwrap();
    let mut path = Path::new(x, 0, &input);
    let (one, two) = path.run();
    println!("1: {}", one);
    println!("2: {}", two);
}

#[derive(Eq,PartialEq,Clone,Copy)]
struct Point { x: usize, y: usize }

struct Path<'a> {
    dir: Dir,
    maze: &'a Vec<Vec<u8>>,
    xy: Point,
    seen: Vec<u8>,
    count: usize,
}

// TODO make this an iterator
impl<'a> Path<'a> {
    fn new(x: usize, y: usize, maze: &'a Vec<Vec<u8>>) -> Self {
        Path {
            dir: Dir::Dn,
            maze,
            xy: Point { x, y },
            seen: Vec::new(),
            count: 1
        }
    }

    fn run(&mut self) -> (&str, usize) {
        while self.step() {}
        (unsafe { ::std::str::from_utf8_unchecked(&self.seen) }, self.count)
    }

    fn step(&mut self) -> bool {
        let mut nd = self.dir;
        macro_rules! try_step {
            ($fnm:ident) => {
                nd = nd.$fnm();
                let np = self.xy + nd;
                let c = self.maze[np.y][np.x];
                if c != b' ' {
                    if (c as char).is_alphabetic() {
                        self.seen.push(c);
                    }
                    self.xy = np;
                    self.dir = nd;
                    self.count += 1;
                    return true;
                }
            }
        }
        try_step!(id);
        try_step!(left);
        try_step!(flip);
        return false;
    }
}

#[derive(Eq,PartialEq,Clone,Copy)]
enum Dir {
    Up,
    Dn,
    Lf,
    Rt,
}

impl Dir {
    fn left(self) -> Self {
        match self {
            Dir::Up => Dir::Lf,
            Dir::Dn => Dir::Rt,
            Dir::Lf => Dir::Dn,
            Dir::Rt => Dir::Up,
        }
    }
    
    fn flip(self) -> Self {
        match self {
            Dir::Up => Dir::Dn,
            Dir::Dn => Dir::Up,
            Dir::Lf => Dir::Rt,
            Dir::Rt => Dir::Lf,
        }
    }

    fn id(self) -> Self { self }
}

impl std::ops::Add<Dir> for Point {
    type Output = Self;
    fn add(self, rhs: Dir) -> Self {
        match rhs {
            Dir::Up => Point { x: self.x + 0, y: self.y - 1 },
            Dir::Dn => Point { x: self.x + 0, y: self.y + 1 },
            Dir::Lf => Point { x: self.x - 1, y: self.y + 0 },
            Dir::Rt => Point { x: self.x + 1, y: self.y + 0 },
        }
    }
}
