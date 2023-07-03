// https://leetcode.com/problems/design-add-and-search-words-data-structure/solutions/3317484/rust-iterative-recursive-on-wildcard/
use std::borrow::Borrow;

#[derive(Default)]
struct WordDictionary {
    end_of_word: bool,
    children: [Option<Box<WordDictionary>>; 26]
}

const A_CODE: usize = 'a' as usize;

fn to_index(c: char) -> usize {
    (c as usize) - A_CODE
}

fn find_slice(str: &str, dict: &WordDictionary) -> bool {
    let mut curr = dict;

    for (i, c) in str.char_indices() {
        match c {
            '.' => {

                for j in 0..curr.children.len() {
                    if let Some(child) = &curr.children[j] {
                        let (_, last) = str.split_at(i + 1);

                        if find_slice(last, child.borrow()) {
                            return true;
                        }
                    }
                }

                return false;
            },
            _ => { 
                match curr.children[to_index(c)] {
                    None => { return false },
                    Some(ref child) => { curr = child }
                }
            },
        }
    }

    curr.end_of_word
}

/** 
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl WordDictionary {

    fn new() -> Self {
        Default::default()
    }
    
    fn add_word(&mut self, word: String) {
        let mut curr = self;

        for i in word.chars().map(to_index) {
            curr = curr.children[i].get_or_insert(Box::new(WordDictionary::new()));
        }

        curr.end_of_word = true;
    }
    
    fn search(&self, word: String) -> bool {
        find_slice(&word, self)
    }
}

/**
 * Your WordDictionary object will be instantiated and called as such:
 * let obj = WordDictionary::new();
 * obj.add_word(word);
 * let ret_2: bool = obj.search(word);
 */