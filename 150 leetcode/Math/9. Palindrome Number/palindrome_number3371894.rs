// https://leetcode.com/problems/palindrome-number/solutions/3371894/rust/
impl Solution {
    pub fn is_palindrome(x: i32) -> bool {
       let s = x.to_string();
       let char_vec: Vec<char> = s.chars().collect();

        let mut i = 0;
        let mut j = char_vec.len() - 1;
        let mut result = true;
        
        loop {
            if (i >= j) {
                break;
            }

            if char_vec[i] != char_vec[j] {
                result = false; 
                break
            }


            i += 1;
            j -= 1;
        }


       result
    }
}