
extern crate aoc;

use std::str::FromStr;
use std::collections::HashMap;

const INPUT: &'static str = "data/day08";

fn main() {
    let input = aoc::file::to_strings(INPUT).into_iter().map(|v| v.parse().unwrap());
    let mut map = HashMap::new();
    let mut highest = ::std::i32::MIN;
    for inst in input {
        process_inst(&mut map, inst, &mut highest);
    }
    let x = map.values().max();
    println!("1: {}", x.unwrap());
    println!("2: {}", highest);
}

fn process_inst(map: &mut HashMap<Reg, i32>, inst: Inst, highest: &mut i32) {
    let run_inst = {
        let ent = map.entry(inst.cond.reg).or_insert(0);
        if *ent > *highest { *highest = *ent }
        inst.cond.comp.run(*ent, inst.cond.amt)
    };
    let ent = map.entry(inst.op.reg).or_insert(0);
    if run_inst {
        *ent += inst.op.amt;
    }
    if *ent > *highest { *highest = *ent }
}

impl Comp {
    fn run<T: Ord>(&self, a: T, b: T) -> bool {
        use Comp::*;
        match *self {
            Gt => a > b, 
            Lt => a < b, 
            Eq => a == b, 
            Neq => a != b, 
            GtEq => a >= b, 
            LtEq=> a <= b, 
        }
    }
}

#[derive(Clone, Copy, PartialEq,Eq)]
enum Comp {
    Gt,
    Lt,
    Eq,
    Neq,
    GtEq,
    LtEq
}

type Reg = String;

#[derive(Clone, PartialEq,Eq)]
struct Op {
    reg: Reg,
    amt: i32,
}

#[derive(Clone, PartialEq,Eq)]
struct Cond {
    reg: Reg,
    comp: Comp,
    amt: i32,
}

#[derive(Clone, PartialEq,Eq)]
struct Inst {
    op: Op,
    cond: Cond,
}

impl FromStr for Op {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut x = s.split_whitespace();
        let reg = x.next().unwrap().into();
        let amt = match x.next() {
            Some(a) => x.next().unwrap().parse::<i32>().unwrap() * if a == "inc" { 1 } else { -1 },
            None => panic!(),
        };
        Ok(Op{ reg, amt })
    }
}

impl FromStr for Comp {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use Comp::*;
        match s {
            ">" => Ok(Gt),
            "<" => Ok(Lt),
            "==" => Ok(Eq),
            "!=" => Ok(Neq),
            ">=" => Ok(GtEq),
            "<=" => Ok(LtEq),
            _ => panic!()
        }
    }
}

impl FromStr for Cond {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut x = s.split_whitespace();
        let reg = x.next().unwrap().into();
        let comp = x.next().unwrap().parse().unwrap();
        let amt = x.next().unwrap().parse().unwrap();
        Ok(Cond{ reg, comp, amt })
    }
}

impl FromStr for Inst {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut x = s.split(" if ");
        let op = x.next().unwrap().parse().unwrap();
        let cond = x.next().unwrap().parse().unwrap();
        Ok(Inst{ op, cond })
    }
}
