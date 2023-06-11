// https://leetcode.com/problems/plus-one/solutions/3470308/rust-solution/
impl Solution {
    pub fn plus_one(digits: Vec<i32>) -> Vec<i32> {
        let mut carry = 1;
        let mut ans = vec![];
        for i in (0..digits.len()).rev() {
            let sum = digits[i] + carry;
            carry = sum / 10;
            ans.push(sum % 10);
        }
        if carry > 0 {
            ans.push(carry);
        }
        ans.reverse();
        ans        
    }
}