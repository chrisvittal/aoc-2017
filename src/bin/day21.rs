
extern crate aoc;
#[macro_use]
extern crate itertools;
extern crate pathfinding;

use std::collections::HashMap;
use std::iter::repeat;

use itertools::Itertools;
use pathfinding::Matrix;

const INPUT: &'static str = "data/day21";

type GridMap = HashMap<Matrix<bool>, (usize, Matrix<bool>)>;

fn extend_map(map: &mut GridMap, s: &str) {
    let (from, to) = s.split(" => ")
        .map(|s| Matrix::square_from_vec(s.bytes().filter_map(|b| match b {
            b'.' => Some(false),
            b'#' => Some(true),
            _ => None,
        }).collect())).next_tuple().unwrap();
    let count = count(&to);
    let it = iproduct!(vec![from.flipped_lr(), from], 0..4)
        .map(|(m, i)| m.rotated_cw(i)).zip(repeat((count, to)));
    map.extend(it);
}

fn count(m: &Matrix<bool>) -> usize {
    m.as_ref().iter().filter(|&b| *b).count()
}

fn run(grid: &mut Matrix<bool>, map: &GridMap) -> usize {
    let mut count = 0;
    let pt_siz = 2 + grid.rows % 2;
    let sl_cnt = grid.rows / pt_siz;
    let mut new_grid = Matrix::new_square(sl_cnt + grid.rows, false);
    for (sq_j, sq_i) in iproduct!(0..sl_cnt, 0..sl_cnt) {
        let (c, ref new) = map[&grid.slice(sq_i*pt_siz..sq_i*pt_siz+pt_siz,
                                      sq_j*pt_siz..sq_j*pt_siz+pt_siz)];
        new_grid.set_slice(&(sq_i*(pt_siz+1), sq_j*(pt_siz+1)), new);
        count += c;
    }
    *grid = new_grid;
    count
}

fn main() {
    const ONE: usize = 5;
    const TWO: usize = 18;
    let start = vec![false,  true, false, false, false,  true, true,  true,  true]; 
    let mut grid = Matrix::square_from_vec(start);
    let mut map = HashMap::new();
    aoc::file::to_lines(INPUT).for_each(|s| extend_map(&mut map, &s.unwrap()));
    let mut count = 0;
    for _ in 0..ONE {
        count = run(&mut grid, &map);
    }
    println!("1: {}", count);
    for _ in ONE..TWO {
        count = run(&mut grid, &map);
    }
    println!("2: {}", count);
}
