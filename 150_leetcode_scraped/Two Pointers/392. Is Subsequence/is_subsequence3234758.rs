// https://leetcode.com/problems/is-subsequence/solutions/3234758/solution-in-rust/
impl Solution {
    pub fn is_subsequence(s: String, t: String) -> bool {
        let mut current_index: usize = 0; 
        let mut n = s.chars().nth(current_index);
        let mut next_letter;
        match n {
            None => return true,
            Some(x) => next_letter = x,
        }
        let mut word:String = String::new();
        for l in t.chars() {
           if current_index  >= s.len() {
               return false; 
           }
           if l == next_letter {
               current_index += 1; 
               next_letter = s.chars().nth(current_index).unwrap_or(l); 
               word.push_str(&l.to_string());
               if word == s {
                   return true;
               }
           }
        }
        return false; 

    }
}