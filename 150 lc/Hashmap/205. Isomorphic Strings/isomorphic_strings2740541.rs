// https://leetcode.com/problems/isomorphic-strings/solutions/2740541/rust-logically-straight-forward-solutions-with-hashmap-or-array/
use std::mem::replace;

impl Solution {
    pub fn is_isomorphic(s: String, t: String) -> bool {
        let mut map = [b'\0'; 256];
        let mut mapped_values = [false; 256];

        s.bytes().zip(t.bytes()).all(|(from, to)| {
            let mapping = &mut map[from as usize];
            if *mapping == b'\0' {
                *mapping = to;
                let is_mapped = &mut mapped_values[to as usize];
                !replace(is_mapped, true)
            } else {
                *mapping == to
            }
        })
    }
}