// https://leetcode.com/problems/plus-one/solutions/3408304/rust-0ms-for-each-insert/
impl Solution {
    pub fn plus_one(digits: Vec<i32>) -> Vec<i32> {
        let mut result = vec![];
        let mut d = 1;
        digits.iter().rev().for_each(|x| {
            let mut s = *x + d;
            if s >= 10 {
                d = 1;
                s %= 10;
            } else {
                d = 0;
            }
            result.insert(0, s)
        });
        if d == 1 {
            result.insert(0,  1);
        }
        result
    }
}