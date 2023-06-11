// https://leetcode.com/problems/ransom-note/solutions/3446362/ransom-note-check-in-rust/
impl Solution {
    pub fn can_construct(
        ransom_note: String, 
        magazine: String
    ) -> bool {
        // Initialize a new array of indices for each lowercase character
        let mut a = [0u16; 26];

        // For each character in the magazine, increase the corresponding index in the array by 1.
        for c in magazine.chars() {
            a[char_to_num(c)] += 1;
        }

        // For each character in the note, check the index isn't 0 (signifying no more available characters).
        for c in ransom_note.chars() {
            if a[char_to_num(c)] > 0 {
                a[char_to_num(c)] -= 1; // Subtract 1 from the index to "use" 1 letter
            } else {
                return false; // If the index is 0 here, the note cannot be constructed.
            }
        }
        // If it makes it through the entire loop, the note is a valid combination of the letters in the magazine.
        return true;
    }
}

// Convert a lowercase char to a number between 0 and 25
pub fn char_to_num(c: char) -> usize {
    c as usize - 'a' as usize
}