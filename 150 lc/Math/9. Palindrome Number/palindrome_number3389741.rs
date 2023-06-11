// https://leetcode.com/problems/palindrome-number/solutions/3389741/rust-simple-solution/
impl Solution {
    pub fn is_palindrome(x: i32) -> bool {
        let v:String = x.to_string();
        let v_vec:Vec<_> = v.chars().collect();
        let v1:Vec<_> = v.chars().rev().collect();

        if v_vec==v1 {
            return true
        } else {
            return false
        }
    }
}