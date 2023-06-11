// https://leetcode.com/problems/is-subsequence/solutions/2815757/short-linear-scan-solution-with-c-and-rust/
impl Solution {
    pub fn is_subsequence(s: String, t: String) -> bool {
        let mut s = s.chars().peekable();
        for c in t.chars() {
            match s.peek() {
                Some(&ch) if ch == c => {
                    s.next();
                },
                _ => {},
            }
        }
		
		// if s is subseq of t, iterator s would be exhausted and 
		// return None on s.next()
        s.next().is_none()
    }
}