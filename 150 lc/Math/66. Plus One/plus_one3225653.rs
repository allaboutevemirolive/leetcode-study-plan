// https://leetcode.com/problems/plus-one/solutions/3225653/rust-most-intiutive/
impl Solution {
    pub fn plus_one(digits: Vec<i32>) -> Vec<i32> {
            let mut carry: i32 = 1;
            let mut digits = digits
                .iter()
                .rev()
                .map(|x| {
                    let sum = *x + carry;
                    carry = sum / 10;
                    temp % 10
                })
                .collect::<Vec<i32>>();
            if carry != 0 {
                digits.push(carry);
            }
            digits.reverse();
            digits
    }
}