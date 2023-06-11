// https://leetcode.com/problems/palindrome-number/solutions/3423132/rust-solution-without-conversion-to-string/
impl Solution {
    pub fn is_palindrome(x: i32) -> bool {
        let mut new_num=0; 
        let mut rem = 0;
        let mut orig_num = x;

        while orig_num>0 {
            rem = orig_num%10;
            orig_num = orig_num/10;
            new_num = new_num*10 + rem;
        }
        if x == new_num{
            true
        }
        else{
            false
        }
    }
}