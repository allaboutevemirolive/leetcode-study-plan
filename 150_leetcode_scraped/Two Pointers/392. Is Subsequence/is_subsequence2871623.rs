// https://leetcode.com/problems/is-subsequence/solutions/2871623/rust-fast-o-n-solution/
impl Solution {
    pub fn is_subsequence(s: String, t: String) -> bool {
        let (mut subsequence,s_len) = (0,s.len());
        let  s = s.chars().collect::<Vec<char>>();
        for t_char in t.chars(){
            if subsequence < s_len && s[subsequence] == t_char {
                    subsequence += 1;
            }
        }
        if subsequence != s_len {
            return false
        }
        true
    }
}