// https://leetcode.com/problems/ransom-note/solutions/3026871/rust-linear-time-using-iterator-fold-and-zip-on-arrays/
impl Solution {
    pub fn can_construct(ransom_note: String, magazine: String) -> bool {

        let ransom_counts = Self::get_letter_counts(&ransom_note);
        let magazine_counts = Self::get_letter_counts(&magazine);

        ransom_counts
            .iter()
            .zip(magazine_counts.iter())
            .all(|(r, m)| r <= m)
    }

    fn get_letter_counts(s: &str) -> [u16; 26] {
        let a_offset = u32::from('a');
        s.chars().fold([0; 26], |mut counts, c| {
            let c_index = c as u32 - a_offset;
            counts[c_index as usize] += 1;
            counts
        })
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    pub fn test_non_matching_letter() {
        let ransom_note = String::from("a");
        let magazine = String::from("b");
        let answer = Solution::can_construct(ransom_note, magazine);
        assert_eq!(false, answer);
    }

    #[test]
    pub fn test_matching_letters() {
        let ransom_note = String::from("a");
        let magazine = String::from("a");
        let answer = Solution::can_construct(ransom_note, magazine);
        assert_eq!(true, answer);
    }

    #[test]
    pub fn test_composition() {
        let ransom_note = String::from("aa");
        let magazine = String::from("aba");
        let answer = Solution::can_construct(ransom_note, magazine);
        assert_eq!(true, answer);
    }

    #[test]
    pub fn test_composition_counts() {
        let ransom_note = String::from("aaa");
        let magazine = String::from("aba");
        let answer = Solution::can_construct(ransom_note, magazine);
        assert_eq!(false, answer);
    }
}