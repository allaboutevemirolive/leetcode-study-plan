// https://leetcode.com/problems/longest-consecutive-sequence/solutions/2239776/rust-two-o-n-variants-with-explanation/
use std::collections::{HashMap, hash_map};
use std::cmp::Ordering;

struct UnionFind {
    root: Vec<usize>,
    rank: Vec<usize>,
}

impl UnionFind {
    pub fn new(n: usize) -> Self {
        Self {
            root: (0..n).collect(),
            rank: vec![1; n],
        }
    }

    pub fn find(&mut self, x: usize) -> usize {
        if self.root[x] == x {
            x
        } else {
            self.root[x] = self.find(self.root[x]);
            self.root[x]
        }
    }

    pub fn union_set(&mut self, x: usize, y: usize) -> bool {
        let root_x = self.find(x);
        let root_y = self.find(y);
        if root_x != root_y {
            match self.rank[root_x].cmp(&self.rank[root_y]) {
                Ordering::Greater => self.root[root_y] = root_x,
                Ordering::Less => self.root[root_x] = root_y,
                Ordering::Equal => {
                    self.root[root_y] = root_x;
                    self.rank[root_x] += 1;
                }
            }
            true
        } else {
            false
        }
    }

    pub fn max_size(&mut self) -> Option<usize> {
        let mut map = HashMap::<usize, usize>::new();
        for i in 0..self.root.len() {
            *map.entry(self.find(i)).or_insert(0) += 1;
        }
        map.into_iter().map(|(_, count)| count).max()
    }

}

impl Solution {
    pub fn longest_consecutive(nums: Vec<i32>) -> i32 {
        let map = {
            let mut i = 0;
            let mut map = HashMap::<i32,usize>::new();
            nums.into_iter().for_each(|num| if let hash_map::Entry::Vacant(e) = map.entry(num) { e.insert(i); i += 1 });
            map
        };

        let mut uf = UnionFind::new(map.len());
        for (num, index) in map.iter() {
            if let Some(right_index) = map.get(&(*num + 1)) {
                uf.union_set(*index, *right_index);
            }
            if let Some(left_index) = map.get(&(*num - 1)) {
                uf.union_set(*index, *left_index);
            }
        }

        uf.max_size().unwrap_or(0) as _
    }
}