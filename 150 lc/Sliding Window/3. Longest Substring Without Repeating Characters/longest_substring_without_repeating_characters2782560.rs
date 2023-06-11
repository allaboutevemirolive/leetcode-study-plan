// https://leetcode.com/problems/longest-substring-without-repeating-characters/solutions/2782560/rust-way-to-solve-it/
fn length_of_longest_substring(s: String) -> i32 {
    let mut chars = std::collections::HashMap::new();
    let (mut res, mut i) = (0, 0);
    for (j, c) in s.char_indices() {
        if let Some(val) = chars.insert(c, j as u32 + 1) {
                i = i32::max(i, val as i32);
        }
        res = i32::max(res, j as i32 - i +1);
    }
    res
}