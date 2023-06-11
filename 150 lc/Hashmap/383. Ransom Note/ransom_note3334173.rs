// https://leetcode.com/problems/ransom-note/solutions/3334173/rust-counter-100/
impl Solution {
    pub fn can_construct(ransom_note: String, magazine: String) -> bool {
        if ransom_note.len() > magazine.len() {
            return false;
        }
        let mut mag_map : [usize; 26] = [0; 26];
        magazine.chars().for_each(|c| mag_map[(c as u8 - b'a') as usize] += 1);
        for c in ransom_note.chars().map(|c| ((c as u8) - b'a') as usize) {
            if mag_map[c] == 0 {
                return false;
            }
            mag_map[c] -= 1;
        }
        true
    }
}