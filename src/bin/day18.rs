
extern crate aoc;

use std::collections::{HashMap, VecDeque};
use std::str::FromStr;

const INPUT: &'static str = "data/day18";

// TODO(cdv): Create a Cpu data structure. To hold all data, notably a copy of 
//            the instructions, registers and program counter. 

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

fn main() {
    let input: Vec<Inst> = aoc::file::to_single_parsed(INPUT);
    let mut comp0 = HashMap::new();
    let mut comp1 = HashMap::new();
    comp0.insert('p', 0);
    comp1.insert('p', 1);

    let mut done0 = false;
    let mut done1 = false;
    let mut ip0 = 0;
    let mut ip1 = 0;
    let mut ss0: VecDeque<i64> = VecDeque::new();
    let mut ss1: VecDeque<i64> = VecDeque::new();
    let mut count = 0;
    let mut res0: Option<char> = None;
    let mut res1: Option<char> = None;

    loop
    {
        if ip0 < 0 || ip0 as usize >= input.len() {
            done0 = true;
        }
        if ip1 < 0 || ip1 as usize >= input.len() {
            done1 = true;
        }
        if done1 { break; }

        if let Some(c) = res0.take() {
            let ent = comp0.entry(c).or_insert(0);
            if let Some(v) = ss1.pop_front() {
                *ent = v;
            } else {
                res0 = Some(c);
            }
        }
        if let Some(c) = res1.take() {
            let ent = comp1.entry(c).or_insert(0);
            if let Some(v) = ss0.pop_front() {
                *ent = v;
            } else {
                res1 = Some(c);
            }
        }

        if res0.is_some() && res1.is_some() { break; }

        if res0.is_none() && !done0 {
            res0 = run(&mut comp0, input[ip0 as usize], &mut ip0,
                       &mut ss1, &mut ss0, &mut 0);
        }

        if res1.is_none() && !done1 {
            res1 = run(&mut comp1, input[ip1 as usize], &mut ip1,
                       &mut ss0, &mut ss1, &mut count);
        }
    }
    println!("{}", count);
}

/// Returns the sound if any was played
fn run(comp: &mut HashMap<char, i64>,
       inst: Inst,
       ip: &mut i64,
       stack_recv: &mut VecDeque<i64>,
       stack_send: &mut VecDeque<i64>,
       sent: &mut u32) -> Option<char> {
    use Inst::*;
    match inst {
        Snd(src) => {
            let ent = comp.entry(src).or_insert(0);
            stack_send.push_back(*ent);
            *sent += 1;
        },
        Set(dst, v) => {
            let ent = comp.entry(dst).or_insert(0);
            *ent = v;
        },
        SetR(dst, src) => {
            let ent_s = *comp.entry(src).or_insert(0);
            let ent_d = comp.entry(dst).or_insert(0);
            *ent_d = ent_s;
        },
        AddReg(dst, src) => {
            let ent_s = *comp.entry(src).or_insert(0);
            let ent_d = comp.entry(dst).or_insert(0);
            *ent_d += ent_s;
        }
        AddLit(dst, v) => {
            let ent = comp.entry(dst).or_insert(0);
            *ent += v;
        },
        MulReg(dst, src) => {
            let ent_s = *comp.entry(src).or_insert(0);
            let ent_d = comp.entry(dst).or_insert(0);
            *ent_d *= ent_s;
        }
        MulLit(dst, v) => {
            let ent = comp.entry(dst).or_insert(0);
            *ent *= v;
        },
        ModReg(dst, src) => {
            let ent_s = *comp.entry(src).or_insert(0);
            let ent_d = comp.entry(dst).or_insert(0);
            *ent_d %= ent_s;
        }
        ModLit(dst, v) => {
            let ent = comp.entry(dst).or_insert(0);
            *ent %= v;
        },
        Rcv(dst) => {
            let ent = comp.entry(dst).or_insert(0);
            if let Some(i) = stack_recv.pop_front() {
                *ent = i;
            } else {
                *ip += 1;
                return Some(dst);
            }
        },
        Jgz(cmp, v) => {
            let ent = comp.entry(cmp).or_insert(0);
            if *ent > 0 {
                *ip += v;
                return None;
            }
        },
        JgzR(cmp, src) => {
            let ent_s = *comp.entry(src).or_insert(0);
            let ent_c = comp.entry(cmp).or_insert(0);
            if *ent_c > 0 {
                *ip += ent_s;
                return None;
            }
        },
        Jun(v) => {
            *ip += v;
            return None;
        },
        Nop => {}
    }
    *ip += 1;
    None
}
