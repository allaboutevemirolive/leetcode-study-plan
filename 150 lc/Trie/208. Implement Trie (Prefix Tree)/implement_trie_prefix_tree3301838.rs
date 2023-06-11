// https://leetcode.com/problems/implement-trie-prefix-tree/solutions/3301838/rust-fast-and-efficient-solution/
use std::collections::HashMap;

struct Trie {
    tree: HashMap<char, Vec<String>>
}

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