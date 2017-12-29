
use std::collections::VecDeque;

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
    tape: VecDeque<bool>,
    pos: usize,
    state: State,
}

impl Machine {
    fn new() -> Self {
        const LEN: usize = 10001;
        Machine {
            tape: vec![false; LEN].into(),
            pos: LEN/2,
            state: State::A,
        }
    }

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
        self.tape[self.pos] = true;
    }

    #[inline(always)]
    fn write_zero(&mut self) {
        self.tape[self.pos] = false;
    }

    #[inline(always)]
    fn val(&self) -> bool {
        self.tape[self.pos]
    }

    #[inline(always)]
    fn left(&mut self) {
        if self.pos == 0 {
            self.tape.push_front(false);
        } else {
            self.pos -= 1;
        }
    }

    #[inline(always)]
    fn right(&mut self) {
        if self.pos == self.tape.len() - 1 {
            self.tape.push_back(false);
        }
        self.pos += 1;
    }

    fn checksum(&self) -> usize {
        self.tape.iter().filter(|&b| *b).count()
    }
}

fn main() {
    let mut machine = Machine::new();
    for _ in 0..12368930 {
        machine.step()
    }

    println!("1: {}", machine.checksum());
    println!("2: Merry Christmas! No part 2!");
}
