
/*! Simple Union-find implementation for advent of code
*/

pub struct UnionFind(Vec<usize>);

impl UnionFind {
    pub fn new(size: usize) -> Self {
        UnionFind((0..size).collect())
    }

    pub fn find(&mut self, idx: usize) -> usize {
        let tmp = self.0[idx];
        if tmp == idx {
            tmp
        } else {
            self.0[idx] = self.find(tmp);
            self.0[idx]
        }
    }

    /// Merges `idx` and `idy`, setting `idx`'s root as the parent to `idy`'s root.
    pub fn union(&mut self, idx: usize, idy: usize) {
        let x = self.find(idx);
        let y = self.find(idy);
        self.0[y] = x;
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }
}
