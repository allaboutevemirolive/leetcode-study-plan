// https://leetcode.com/problems/is-subsequence/solutions/3562379/rust-beats-100-no-loops/
use std::collections::HashMap;

impl Solution {
    pub fn is_subsequence(s: String, t: String) -> bool {
        let mut t = t;
        let mut s = s;
        t.retain(|c| s.contains(c));

        if t.len() < s.len() {return false}

        let mut previous:Option<char> = None;
        t.retain(|x| Some(x) != std::mem::replace(&mut previous, Some(x)));
        
        previous= None;
        s.retain(|x| Some(x) != std::mem::replace(&mut previous, Some(x)));
        t.matches(&s).collect::<Vec<&str>>().len() > 0 
    }
}