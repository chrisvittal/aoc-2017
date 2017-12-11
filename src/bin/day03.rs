
const INPUT: u32 = 312051;

fn main() {
    let mut spiral = SpiralIterator::new();
    while spiral.ent.n < INPUT {
        spiral.next();
    }
    println!("1: {}", spiral.ent.x.abs() + spiral.ent.y.abs());
    let mut spiral2 = SpiralIterator2::new();
    while let Some(v) = spiral2.next() {
        if v > INPUT {
            println!("2: {}", v);
            break;
        }
    }
}

#[derive(Debug, Clone, Copy)]
struct State<T: Ord> {
    max_x: T,
    min_x: T,
    max_y: T,
    min_y: T,
    dir: Direction,
}

impl<T: Ord> State<T> {
    fn update(mut self, x: T, y: T) -> Self {
        if x > self.max_x {
            self.max_x = x;
            self.dir = self.dir.turn_left();
        } else if x < self.min_x {
            self.min_x = x;
            self.dir = self.dir.turn_left();
        } else if y > self.max_y {
            self.max_y = y;
            self.dir = self.dir.turn_left();
        } else if y < self.min_y {
            self.min_y = y;
            self.dir = self.dir.turn_left();
        }
        self
    }
}

impl<T: Ord + Clone> State<T> {
    fn new(it: &T) -> Self {
        State {
            max_x: it.clone(),
            min_x: it.clone(),
            max_y: it.clone(),
            min_y: it.clone(),
            dir: Direction::Right,
        }
    }
}

#[derive(Debug, Clone, Copy)]
struct SpiralEntry {
    x: i32,
    y: i32,
    n: u32,
}

#[derive(Debug, Clone, Copy)]
struct SpiralIterator {
    ent: SpiralEntry,
    state: State<i32>,
}

impl SpiralIterator {
    fn new() -> Self {
        SpiralIterator {
            ent: SpiralEntry { x: 0, y: 0, n: 1 },
            state: State::new(&0),
        }
    }
}

#[derive(Clone)]
struct SpiralIterator2 {
    x: usize,
    y: usize,
    state: State<usize>,
    backing: Vec<Vec<u32>>
}

impl SpiralIterator2 {
    fn new() -> Self {
        let mut v = Vec::new();
        for _ in 0..1024 {
            v.push(vec![0; 1024]);
        }
        let mut ret = SpiralIterator2 {
            x: 512,
            y: 512,
            state: State::new(&512),
            backing: v
        };
        ret.backing[ret.x][ret.y] = 1;
        ret
    }
}

#[derive(Debug, Clone, Copy)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    fn turn_left(&self) -> Self {
        use Direction::*;
        match *self {
            Up => Left,
            Left => Down,
            Down => Right,
            Right => Up,
        }
    }
}

impl Iterator for SpiralIterator {
    type Item = SpiralEntry;

    fn next(&mut self) -> Option<Self::Item> {
        use Direction::*;
        let ret = self.ent;
        self.ent.n += 1;
        match self.state.dir {
            Up => self.ent.y += 1,
            Left => self.ent.x -= 1,
            Down => self.ent.y -= 1,
            Right => self.ent.x += 1,
        }
        self.state = self.state.update(self.ent.x, self.ent.y);
        Some(ret)
    }
}

impl Iterator for SpiralIterator2 {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        use Direction::*;
        let ret = fill(&mut *self.backing, self.x, self.y);
        match self.state.dir {
            Up => self.y += 1,
            Left => self.x -= 1,
            Down => self.y -= 1,
            Right => self.x += 1,
        }
        self.state = self.state.update(self.x, self.y);
        Some(ret)
    }
}

fn fill(buf: &mut [Vec<u32>], x: usize, y: usize) -> u32 {
    let mut tmp = 0;
    for p in [-1, 0, 1].iter() {
        for q in [-1, 0, 1].iter() {
            tmp += buf[(x as isize + p) as usize][(y as isize + q) as usize]
        }
    }
    buf[x][y] = tmp;
    tmp
}

