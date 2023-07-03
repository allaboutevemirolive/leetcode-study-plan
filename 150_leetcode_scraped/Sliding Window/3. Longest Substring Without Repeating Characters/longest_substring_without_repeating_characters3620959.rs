// https://leetcode.com/problems/longest-substring-without-repeating-characters/solutions/3620959/simple-rust-solution/
impl Solution {
    pub fn length_of_longest_substring(S: String) -> i32 {
   use std::cmp::max;
    use std::collections::HashSet;
    let mut set: HashSet<char> = HashSet::new();
    // since string can not indexed in rust, store it in vec
    let char_vec: Vec<char> = S.chars().collect();
    let (mut i, mut j) = (0 as usize, 0 as usize);
    let mut ans = 0;
    while j < S.len() {
        if let Some(c) = set.get(&char_vec[j]) {
            ans = max(ans, j - i);
            set.remove(&char_vec[i]);
            i += 1;
        } else {
            set.insert(char_vec[j]);
            j += 1;
            ans = max(ans, j - i);
        }
    }
    return ans as i32;

        
    }
}