// https://leetcode.com/problems/valid-palindrome/solutions/3469957/rust-solution-beats-100/
impl Solution {
    pub fn is_palindrome(s: String) -> bool {
    let chrs : Vec<char> = s.chars().collect();
    let mut forward_ptr = 0;
    let mut backward_ptr = s.len()-1;
    let mut is_valid = true;

    while  forward_ptr < backward_ptr {
        let a : char = chrs[forward_ptr];
        let b : char = chrs[backward_ptr];

        if !a.is_alphanumeric() {
            forward_ptr +=1;
        } else if !b.is_alphanumeric() {
            backward_ptr -=1;
        } else if a.to_ascii_lowercase() != b.to_ascii_lowercase(){
            is_valid = false;
            break;
        } else {
            forward_ptr +=1;
            backward_ptr -=1;
        }
    }
    is_valid
}
}