// https://leetcode.com/problems/valid-palindrome/solutions/3358346/rust-fast-and-simple/
impl Solution {
    pub fn is_palindrome(s: String) -> bool{
        let mut test = s.to_lowercase();
        test.retain(|x| (x as char).is_alphanumeric());
        let res = test.chars().rev().collect::<String>();
        if res == test {
            return true
        }
        false
    }
}