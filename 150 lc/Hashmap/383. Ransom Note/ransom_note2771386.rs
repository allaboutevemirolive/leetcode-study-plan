// https://leetcode.com/problems/ransom-note/solutions/2771386/quick-rust/
impl Solution {
    pub fn can_construct(ransom_note: String, magazine: String) -> bool {
        let mut freq_table = vec![0; 26];
        magazine
            .chars()
            .for_each(|chr| freq_table[(chr as u8 - 97) as usize] += 1);

        ransom_note
            .chars()
            .for_each(|chr| freq_table[(chr as u8 - 97) as usize] -= 1);

        for count in freq_table {
            if count < 0 {
                return false;
            }
        }
        true
    }
}