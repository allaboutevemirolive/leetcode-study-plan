// https://leetcode.com/problems/ransom-note/solutions/2567385/not-great-but-understandable-and-readable-rust-solution/
use std::collections::HashMap;

impl Solution {
    pub fn can_construct(ransom_note: String, magazine: String) -> bool {
        // use a map and count the letter in the magazine
        // then do -1 each time a letter is used in the ransom note
        // if the value for a letter key reaches -x then false
        // if a key is not fund then false
        // else true
        let mut count_letters: HashMap<char, i32> = HashMap::new();
        for letters in magazine.chars() {
             count_letters.entry(letters).and_modify(|counter| *counter += 1).or_insert(1);
        }
        for note in ransom_note.chars(){
            if !count_letters.contains_key(&note){
                return false;
            }
            count_letters.entry(note).and_modify(|counter| *counter -= 1);
        }
        if count_letters.values().any(|&x| x < 0){
            return false;
        } else{
            return true;
        }
    }
}