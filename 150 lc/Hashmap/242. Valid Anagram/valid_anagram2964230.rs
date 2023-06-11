// https://leetcode.com/problems/valid-anagram/solutions/2964230/rust-0ms-array/
impl Solution {
    pub fn is_anagram(s: String, t: String) -> bool {
        // if strings are of different size, anagram impossible
        if s.len() != t.len() { return false; }

        // frequency of each bytes or characters (as the encoding is ASCII for lower case english letters)
        let mut freq = [0; 26];

        // store the frequency in string s
        for c in s.as_bytes() {
            freq[(c - b'a') as usize] += 1;
        }

        // check there are enough letters in s to obtain t
        for c in t.as_bytes() {
            freq[(c - b'a') as usize] -= 1;
        }

        // check if the frequencies match
        for f in freq {
            if f != 0 { return false; }
        }
        
        true
    }
}