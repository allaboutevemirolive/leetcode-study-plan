// https://leetcode.com/problems/isomorphic-strings/solutions/3589326/rust-solution-beat-100-in-time-and-93-in-memory/
impl Solution {
    pub fn is_isomorphic(s: String, t: String) -> bool {
        let mut ms = [0 as u8; 128];
        let mut mt = [0 as u8; 128];
        let s = s.as_bytes();
        let t = t.as_bytes();
        for i in 0..s.len() {
            let sc = s[i] as usize;
            let tc = t[i] as usize;
            if ms[sc] == 0 && mt[tc] == 0 {
                ms[sc] = tc as u8;
                mt[tc] = sc as u8;
            } else if ms[sc] != tc as u8 ||  mt[tc] != sc as u8 {
                return false;
            }
        }
        true
    }
}