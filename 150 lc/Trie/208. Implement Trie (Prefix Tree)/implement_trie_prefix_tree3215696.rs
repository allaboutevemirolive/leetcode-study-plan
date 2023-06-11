// https://leetcode.com/problems/implement-trie-prefix-tree/solutions/3215696/functional-rust-solution-using-str/
use std::boxed::Box;

#[derive(Default)]
struct Trie {
    children: [Option<Box<Trie>>; 26],
    end_of_word: bool,
}

fn byte_to_index(byte: u8) -> usize {
    (byte - b'a') as usize
}

impl Trie {
    fn new() -> Self {
        Trie::default()
    }

    fn set_end_of_word(&mut self) {
        self.end_of_word = true;
    }

    fn insert_str(&mut self, word: &str) {
        word.as_bytes().first().map(|letter| {
            let index = byte_to_index(*letter);
            if self.children[index].is_none() {
                self.children[index] = Some(Box::new(Trie::new()));
            }   
            if word.len() == 1 {
                self.children[index].as_mut().unwrap().set_end_of_word();
            }
            self.children[index].as_mut().unwrap().insert_str(&word[1..]);
        });
    } 
    
    fn insert(&mut self, word: String) {
        self.insert_str(&word);
    }
    
    // (search, starts_with)
    fn search_generic(&self, word: &str) -> (bool, bool) {
        word.as_bytes().first().map(|letter| {
            let index = byte_to_index(*letter);
            self.children[index].as_ref().map(|node| {
                if word.len() == 1 {
                    (node.end_of_word, true)
                } else {
                    node.search_generic(&word[1..])
                }
            }).unwrap_or((false, false))
        }).unwrap_or((false, false))
        // unwrap_or allows us to unpack boolean values from inside the
        // Option returned from map() and fall back to false values
    }

    fn search(&self, word: String) -> bool {
        self.search_generic(&word).0
    }
    
    fn starts_with(&self, prefix: String) -> bool {
        self.search_generic(&prefix).1
    }
}
