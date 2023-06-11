// https://leetcode.com/problems/ransom-note/solutions/3280044/rust-solution/
impl Solution {
    pub fn can_construct(ransom_note: String, magazine: String) -> bool {
        let mut dict = vec![0; 26];
        for c in magazine.chars() {
            dict[c as usize - 'a' as usize] += 1;
        }
        for c in ransom_note.chars() {
            let d = &mut dict[c as usize - 'a' as usize];
            if *d == 0 {
                return false;
            }
            *d -= 1;
        }
        true        
    }
}