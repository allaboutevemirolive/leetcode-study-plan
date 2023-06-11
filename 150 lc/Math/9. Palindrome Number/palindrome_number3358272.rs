// https://leetcode.com/problems/palindrome-number/solutions/3358272/rust-solution/
impl Solution {
    pub fn is_palindrome(x: i32) -> bool {
        
        let mut rev:i32 = 0;
        let mut input = x;

        if input < 0 {
            return false
        }

        while input != 0 {
            rev = rev * 10 + input % 10;
            input = input/10; 
        }

        return x == rev 
    }
}