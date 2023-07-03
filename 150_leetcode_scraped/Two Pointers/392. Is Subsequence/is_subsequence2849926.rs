// https://leetcode.com/problems/is-subsequence/solutions/2849926/functional-style-rust-implementation-using-iterators-1ms/
impl Solution {
    pub fn is_subsequence(s: String, t: String) -> bool {
        let mut cur = 0;
        s.chars()
            .all(|e|
                t[cur..]
                    .find(e)
                    .and_then(|i| { cur += i+1; Some(i) })
                    .is_some()
            )
    }
}