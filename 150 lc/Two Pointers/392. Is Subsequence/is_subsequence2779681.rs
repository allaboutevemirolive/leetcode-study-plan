// https://leetcode.com/problems/is-subsequence/solutions/2779681/rust-0-ms-runtime-beats-100-2-mb-memory-beats-97-45/
fn subseq(s: &str, t: &str) -> bool {
    let sc = s.chars().next();
    let tc = t.chars().next();

    match (sc, tc) {
        (None, _) => true,
        (Some(_), None) => false,
        (Some(ssc), Some(ttc)) => {

            if ssc == ttc {
                subseq(&s[1..], &t[1..])
            } else {
                subseq(s, &t[1..])
            }
            
        }
    }
}


impl Solution {
    pub fn is_subsequence(s: String, t: String) -> bool {
        
        subseq(&s, &t)
        
    }
}