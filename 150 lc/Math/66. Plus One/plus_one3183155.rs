// https://leetcode.com/problems/plus-one/solutions/3183155/rust-minus-one-hack/
impl Solution {
    pub fn plus_one(digits: Vec<i32>) -> Vec<i32> {
        let mut res = digits;
        for i in (-1..res.len() as i32).rev() {
            if (i < 0) {
                res.insert(0, 1);
                break;
            }
            res[i as usize] += 1;
            if res[i as usize] <= 9 {
                break;
            }
            res[i as usize] = 0
        }
        return res;        
    }
}