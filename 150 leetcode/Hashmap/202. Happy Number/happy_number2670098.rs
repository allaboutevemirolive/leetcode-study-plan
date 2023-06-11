// https://leetcode.com/problems/happy-number/solutions/2670098/rust-0ms-btreeset/
use std::collections::BTreeSet;

impl Solution {
    pub fn is_happy(n: i32) -> bool {
        let mut s = ssd(n);
        let mut seen = BTreeSet::new();
        loop {
            if s == 1 { return true }
            /*
            Calling insert on a BTreeSet returns true if
            the input already exists in the collection. 
            Otherwise add to collection and return false.
            */
            if !seen.insert(s) { return false }
            s = ssd(s)
        } false
    }
}

fn ssd(n: i32) -> i32 {
    // Here we use fold to add up the sum of squares
    // Definitely look into fold if you're unfamiliar
    n.to_string().chars().fold(0, | acc, d| acc + d.to_digit(10).unwrap().pow(2) as i32)
}