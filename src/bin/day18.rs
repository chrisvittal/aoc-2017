
extern crate aoc;

use std::collections::{HashMap, VecDeque};
use std::str::FromStr;

const INPUT: &'static str = "data/day18";

fn main() {
    let input: Vec<Inst> = aoc::file::to_single_parsed(INPUT);
    let mut cpu0 = Cpu::from_instructions(0, input);
    let tmp = loop {
        match cpu0.step(false) {
            Some(i) => break i,
            _ => continue,
        }
    };
    println!("1: {}", tmp);
    cpu0.reset();
    let mut cpu1 = cpu0.with_new_id(1);
    cpu1.reset();
    loop
    {
        if cpu1.finished { break; }

        if let Some(v) = cpu0.step(true) {
            cpu1.in_queue.push_back(v);
        }

        if let Some(v) = cpu1.step(true) {
            cpu0.in_queue.push_back(v);
        }

        if cpu0.blocked && cpu1.blocked { break; }

        if let Some(v) = cpu0.step(true) {
            cpu1.in_queue.push_back(v);
        }

        if let Some(v) = cpu1.step(true) {
            cpu0.in_queue.push_back(v);
        }

    }
    println!("2: {}", cpu1.sent);
}

#[derive(Clone)]
struct Cpu {
    regs: HashMap<char, i64>,
    insts: Vec<Inst>,
    pc: i64,
    in_queue: VecDeque<i64>,
    sent: usize,
    id: u8,
    blocked: bool,
    finished: bool,
}

impl Cpu {
    fn from_instructions(cpu_id: u8, insts: Vec<Inst>) -> Self {
        Cpu {
            regs: HashMap::new(),
            insts: insts,
            pc: 0,
            in_queue: VecDeque::new(),
            sent: 0,
            id: cpu_id,
            blocked: false,
            finished: false,
        }
    }

    /// Clones the cpu, but gives a new id, `nid` must be different from `self.id`.
    fn with_new_id(&self, nid: u8) -> Self {
        assert!(self.id != nid);
        let mut ncpu = self.clone();
        ncpu.id = nid;
        ncpu
    }

    fn reset(&mut self) {
        self.regs.values_mut().for_each(|v| *v = 0);
        self.pc = 0;
        self.in_queue.clear();
        self.sent = 0;
        self.blocked = false;
        self.finished = false;
        *self.regs.entry('p').or_insert(0) = self.id as i64;
    }
}

impl Cpu {
    fn step(&mut self, part2: bool) -> Option<i64> {
        use Inst::*;
        if self.finished { return None; }
        match self.insts[self.pc as usize] {
            Snd(src) => {
                let ent = self.regs.entry(src).or_insert(0);
                if part2 {
                    self.sent += 1;
                    self.pc += 1;
                    return Some(*ent);
                } else {
                    self.in_queue.push_back(*ent);
                }
            },
            Set(dst, v) => {
                let ent = self.regs.entry(dst).or_insert(0);
                *ent = v;
            },
            SetR(dst, src) => {
                let ent_s = *self.regs.entry(src).or_insert(0);
                let ent_d = self.regs.entry(dst).or_insert(0);
                *ent_d = ent_s;
            },
            AddReg(dst, src) => {
                let ent_s = *self.regs.entry(src).or_insert(0);
                let ent_d = self.regs.entry(dst).or_insert(0);
                *ent_d += ent_s;
            }
            AddLit(dst, v) => {
                let ent = self.regs.entry(dst).or_insert(0);
                *ent += v;
            },
            MulReg(dst, src) => {
                let ent_s = *self.regs.entry(src).or_insert(0);
                let ent_d = self.regs.entry(dst).or_insert(0);
                *ent_d *= ent_s;
            }
            MulLit(dst, v) => {
                let ent = self.regs.entry(dst).or_insert(0);
                *ent *= v;
            },
            ModReg(dst, src) => {
                let ent_s = *self.regs.entry(src).or_insert(0);
                let ent_d = self.regs.entry(dst).or_insert(0);
                *ent_d %= ent_s;
            }
            ModLit(dst, v) => {
                let ent = self.regs.entry(dst).or_insert(0);
                *ent %= v;
            },
            Rcv(dst) => {
                let ent = self.regs.entry(dst).or_insert(0);
                if part2 {
                    if let Some(i) = self.in_queue.pop_front() {
                        self.blocked = false;
                        *ent = i;
                    } else {
                        self.blocked = true;
                        return None;
                    }
                } else {
                    if *ent != 0 {
                        self.pc += 1;
                        return self.in_queue.pop_back();
                    }
                }
            },
            Jgz(cmp, v) => {
                let ent = self.regs.entry(cmp).or_insert(0);
                if *ent > 0 {
                    self.pc += v;
                    return None;
                }
            },
            JgzR(cmp, src) => {
                let ent_s = *self.regs.entry(src).or_insert(0);
                let ent_c = self.regs.entry(cmp).or_insert(0);
                if *ent_c > 0 {
                    self.pc += ent_s;
                    return None;
                }
            },
            Jun(v) => {
                self.pc += v;
                return None;
            },
            Nop => {}
        }
        self.pc += 1;
        if self.pc < 0 || self.pc as usize >= self.insts.len() { self.finished = true; }
        None
    }
}

#[derive(Clone,Eq,Copy,PartialEq,Debug)]
enum Inst {
    Snd(char),
    Set(char, i64),
    SetR(char, char),
    AddReg(char, char),
    AddLit(char, i64),
    MulReg(char, char),
    MulLit(char, i64),
    ModReg(char, char),
    ModLit(char, i64),
    Rcv(char),
    Jgz(char, i64),
    JgzR(char,char),
    Jun(i64),
    Nop
}

impl FromStr for Inst {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, ()> {
        use Inst::*;
        let mut it = s.split_whitespace();
        match it.next().unwrap() {
            "snd" => return Ok(Snd(it.next().unwrap().chars().nth(0).unwrap())),
            "set" => {
                let c = it.next().unwrap().chars().nth(0).unwrap();
                let i = it.next().unwrap();
                if let Ok(i) = i.parse() {
                    return Ok(Set(c,i));
                } else {
                    return Ok(SetR(c,i.chars().nth(0).unwrap()));
                }
            },
            "add" => {
                let c = it.next().unwrap().chars().nth(0).unwrap();
                let t = it.next().unwrap();
                if let Ok(i) = t.parse() {
                    return Ok(AddLit(c, i));
                } else {
                    return Ok(AddReg(c, t.chars().nth(0).unwrap()));
                }
            }
            "mul" => {
                let c = it.next().unwrap().chars().nth(0).unwrap();
                let t = it.next().unwrap();
                if let Ok(i) = t.parse() {
                    return Ok(MulLit(c, i));
                } else {
                    return Ok(MulReg(c, t.chars().nth(0).unwrap()));
                }
            }
            "mod" => {
                let c = it.next().unwrap().chars().nth(0).unwrap();
                let t = it.next().unwrap();
                if let Ok(i) = t.parse() {
                    return Ok(ModLit(c, i));
                } else {
                    return Ok(ModReg(c, t.chars().nth(0).unwrap()));
                }
            }
            "rcv" => return Ok(Rcv(it.next().unwrap().chars().nth(0).unwrap())),
            "jgz" => {
                let t = it.next().unwrap();
                let i = it.next().unwrap();
                if let Ok(j) = t.parse::<i64>() {
                    if j > 0 {
                        return Ok(Jun(i.parse().unwrap())); // okay for input
                    } else {
                        return Ok(Nop);
                    }
                } else {
                    let c = t.chars().nth(0).unwrap();
                    if let Ok(i) = i.parse() {
                        return Ok(Jgz(c,i));
                    } else {
                        return Ok(JgzR(c,i.chars().nth(0).unwrap()));
                    }
                }
            },
            _ => panic!()
        }
    }
}

