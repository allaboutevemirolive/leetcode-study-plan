// https://leetcode.com/problems/implement-trie-prefix-tree/solutions/3306661/rust-faster-100-100-unsig-first-prefix/
use std::collections::HashMap;

struct Trie {
    tree: HashMap<char, Vec<String>>
}


/**
  * `&self` means the method takes an immutable reference
  * If you need a mutable reference, change it to `&mut self` instead.
 */
impl Trie {

    fn new() -> Self {
        Trie{tree:HashMap::new()}
    }

    fn insert(&mut self, word: String) {
        self.tree.entry(word.chars().next().unwrap()).or_default().push(word);
    }

    fn search(&self, word: String) -> bool {
        if let Some(vals) = self.tree.get(&word.chars().next().unwrap()) {
            return vals.contains(&word)
        }
        false
    }

    fn starts_with(&self, prefix: String) -> bool {
        if let Some(vals) = self.tree.get(&prefix.chars().next().unwrap()) {
            return vals.iter().any(|word| word.starts_with(&prefix))
        }
        false
    }
}

/**
 * Your Trie object will be instantiated and called as such:
 * let obj = Trie::new();
 * obj.insert(word);
 * let ret_2: bool = obj.search(word);
 * let ret_3: bool = obj.starts_with(prefix);
 */