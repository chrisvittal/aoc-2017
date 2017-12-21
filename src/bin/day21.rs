
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

struct Grid<'a> {
    grid: Matrix<bool>,
    map: &'a GridMap
}

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

#[inline]
fn count(m: &Matrix<bool>) -> usize {
    m.as_ref().iter().filter(|&b| *b).count()
}

impl<'a> Grid<'a> {
    #[inline]
    fn run(&mut self) -> usize {
        let mut count = 0;
        let pt_siz = 2 + self.grid.rows % 2;
        let sl_cnt = self.grid.rows / pt_siz;
        let mut new_grid = Matrix::new_square(sl_cnt + self.grid.rows, false);
        for (sq_j, sq_i) in iproduct!(0..sl_cnt, 0..sl_cnt) {
            let (c, ref new) =
                self.map[&self.grid.slice(sq_i*pt_siz..sq_i*pt_siz+pt_siz,
                                          sq_j*pt_siz..sq_j*pt_siz+pt_siz)];
            new_grid.set_slice(&(sq_i*(pt_siz+1), sq_j*(pt_siz+1)), new);
            count += c;
        }
        self.grid = new_grid;
        count
    }
}

impl<'a> Iterator for Grid<'a> {
    type Item = usize;
    fn next(&mut self) -> Option<usize> {
        Some(self.run())
    }
}

fn main() {
    const ONE: usize = 5;
    const TWO: usize = 18;
    let mut map = HashMap::new();
    aoc::file::to_lines(INPUT).for_each(|s| extend_map(&mut map, &s.unwrap()));

    let start = vec![false,  true, false, false, false,  true, true,  true,  true];
    let grid = Matrix::square_from_vec(start);
    let mut grid = Grid { grid: grid, map: &map };
    println!("1: {}", grid.nth(ONE-1).unwrap());
    println!("2: {}", grid.nth(TWO-ONE-1).unwrap());
}
