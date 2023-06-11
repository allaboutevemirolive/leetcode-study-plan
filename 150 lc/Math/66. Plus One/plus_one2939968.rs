// https://leetcode.com/problems/plus-one/solutions/2939968/rust-simple-classic-solution/
impl Solution {
    pub fn plus_one(mut digits: Vec<i32>) -> Vec<i32> {
        for x in digits.iter_mut().rev() {
            *x = (*x + 1) % 10;
            if *x != 0 { return digits; }
        }
        digits.insert(0, 1);
        digits       
    }
}