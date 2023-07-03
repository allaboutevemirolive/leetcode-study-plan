// https://leetcode.com/problems/palindrome-number/solutions/3165894/rust-canonical-nothing-fancy/
impl Solution {
    pub fn is_palindrome(x: i32) -> bool {
        if x<0 {
            return false;
        }
        let mut y = x;
        let mut z: i32 = 0;
        while y>0 {
            z = z*10 + y%10;
            y/=10;
        }
        z==x
    }
}