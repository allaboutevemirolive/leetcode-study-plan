// https://leetcode.com/problems/longest-substring-without-repeating-characters/solutions/3153769/rust-sliding-window-solution/
impl Solution {
    pub fn length_of_longest_substring(s: String) -> i32 {
        let n = s.len();
        let bytes = s.as_bytes();
        let mut hash = vec![0; 256];

        (0..n)
            .fold((0, 0), |(mut r, mut longest), l| {
                while r < n && hash[bytes[r] as usize] == 0 {
                    hash[bytes[r] as usize] = 1;
                    r += 1;
                }

                longest = longest.max(r - l);
                hash[bytes[l] as usize] = 0;

                (r, longest)
            }).1 as i32
    }
}