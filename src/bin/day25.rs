
use std::collections::HashSet;

#[derive(Clone,Copy)]
enum State {
    A,
    B,
    C,
    D,
    E,
    F
}

struct Machine {
    tape: HashSet<i32>,
    pos: i32,
    state: State,
}

impl Machine {
    fn step(&mut self) {
        use State::*;
        match (self.state, self.val()) {
            (A, false) => {
                self.write_one();
                self.right();
                self.state = B;
            },
            (A, true) => {
                self.write_zero();
                self.right();
                self.state = C;
            },
            (B, false) => {
                self.write_zero();
                self.left();
                self.state = A;
            }
            (B, true) => {
                self.write_zero();
                self.right();
                self.state = D;
            },
            (C, false) => {
                self.write_one();
                self.right();
                self.state = D;
            }
            (C, true) => {
                self.write_one();
                self.right();
                self.state = A;
            },
            (D, false) => {
                self.write_one();
                self.left();
                self.state = E;
            }
            (D, true) => {
                self.write_zero();
                self.left();
                self.state = D;
            },
            (E, false) => {
                self.write_one();
                self.right();
                self.state = F;
            }
            (E, true) => {
                self.write_one();
                self.left();
                self.state = B;
            },
            (F, false) => {
                self.write_one();
                self.right();
                self.state = A;
            },
            (F, true) => {
                self.write_one();
                self.right();
                self.state = E;
            },
        }
    }

    #[inline(always)]
    fn write_one(&mut self) {
        self.tape.insert(self.pos);
    }

    #[inline(always)]
    fn write_zero(&mut self) {
        self.tape.remove(&self.pos);
    }

    #[inline(always)]
    fn val(&self) -> bool {
        self.tape.contains(&self.pos)
    }

    #[inline(always)]
    fn left(&mut self) {
        self.pos -= 1;
    }

    #[inline(always)]
    fn right(&mut self) {
        self.pos += 1;
    }

    fn checksum(&self) -> usize {
        self.tape.len()
    }
}

fn main() {
    let mut machine = Machine {
        tape: HashSet::with_capacity(5000),
        pos: 0,
        state: State::A,
    };
    for _ in 0..12368930 {
        machine.step()
    }

    println!("1: {}", machine.checksum());
    println!("2: Merry Christmas! No part 2!");
}
