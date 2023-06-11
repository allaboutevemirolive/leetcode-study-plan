// https://leetcode.com/problems/longest-substring-without-repeating-characters/solutions/2849836/rust-0ms-using-buffer-to-store-current-longest-substring/
impl Solution {
    pub fn length_of_longest_substring(s: String) -> i32 {
        let mut longest = 0;
        let mut buffer = Vec::new();

        for c in s.chars() {
            let c_index = buffer.iter().position(|&buf_c| buf_c == c);

            if let Some(i) = c_index {
                buffer.drain(0..i + 1);
            }

            buffer.push(c);

            if buffer.len() > longest {
                longest = buffer.len();
            }
        }

        longest as i32
    }
}