// https://leetcode.com/problems/longest-substring-without-repeating-characters/solutions/3296850/blazingly-fast-rust/
impl Solution {
      pub fn length_of_longest_substring(s: String) -> i32 {
        let chars = s.as_bytes();

        let mut max_length: i32 = 0;
        let mut letters: u128 = 0;

        let mut start = 0;
        let mut end = 0;

        while end < chars.len() {
            if letters & 1 << chars[end] != 0 {
                while chars[start] != chars[end] {
                    letters ^= 1 << chars[start];
                    start += 1;
                }
                start += 1;
            }
            letters |= 1 << chars[end];
            end += 1;
            max_length = std::cmp::max(max_length, (end - start) as i32);
        }
        max_length
    }
}