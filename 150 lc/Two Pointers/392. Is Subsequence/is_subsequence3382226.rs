// https://leetcode.com/problems/is-subsequence/solutions/3382226/counter-rust-solution/
impl Solution {
    pub fn is_subsequence(s: String, t: String) -> bool {
        let s: Vec<char> = s.chars().collect();
        let t: Vec<char> = t.chars().collect();

        let mut cnt = 0;
        for c in t.iter() {
            if cnt >= s.len() { return true }

            if *c == s[cnt] { cnt += 1 }
        }
        cnt == s.len()
    }
}