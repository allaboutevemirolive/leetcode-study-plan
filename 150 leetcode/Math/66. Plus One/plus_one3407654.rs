// https://leetcode.com/problems/plus-one/solutions/3407654/rust-0ms/
impl Solution {
    pub fn plus_one(digits: Vec<i32>) -> Vec<i32> {
        let mut d = 1;
        let mut v: Vec<i32> = digits
        .iter()
        .rev()
        .map(|x|
            {
                let s = d + *x;
                if s == 10 {
                    d = 1;
                    0
                } else {
                    d = 0;
                    s
                }
            }
        )
        .collect();
        if d == 1 {
            v.push(1);
        }
        v.iter().rev().map(|x| *x).collect()
    }
}