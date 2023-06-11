// https://leetcode.com/problems/valid-palindrome/solutions/3348255/rust-solution/
impl Solution {
    pub fn is_palindrome(s: String) -> bool {

        let mut t = s.to_uppercase();
        t.retain(|c| c.is_alphanumeric());
            
        let n = t.len();
        for i in 0..(n/2) {
            if &t[i..(i+1)] != &t[(n-i-1)..(n-i)] {
                return false;
            }
        }
        return true;      
    }
}