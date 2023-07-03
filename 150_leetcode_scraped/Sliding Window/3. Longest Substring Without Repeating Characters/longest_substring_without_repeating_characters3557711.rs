// https://leetcode.com/problems/longest-substring-without-repeating-characters/solutions/3557711/rust-sliding-window-with-hashset/
impl Solution {
    pub fn length_of_longest_substring(s: String) -> i32 {
        let mut window_start = 0;
        let chars: Vec<char> = s.chars().collect();
        let mut set = std::collections::HashSet::new();
        let mut length = 0;
        for window_end in 0..chars.len() {
            while set.contains(&chars[window_end]) {
                set.remove(&chars[window_start]);
                window_start += 1;
            }
            set.insert(&chars[window_end]);
            length = std::cmp::max(length, window_end - window_start + 1);
        }
        length as i32
    }
}