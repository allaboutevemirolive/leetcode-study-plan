// https://leetcode.com/problems/implement-trie-prefix-tree/solutions/3309010/rust-26ms-14-4-mb/
use std::collections::HashMap;

struct TrieNode {
    is_complete_word: bool,
    children: HashMap<char, TrieNode>,
}

impl TrieNode {
    fn new() -> Self {
        TrieNode {
            is_complete_word: false,
            children: HashMap::new(),
        }
    }
}

struct Trie {
    root: TrieNode,
}

impl Trie {
    fn new() -> Self {
        Trie {
            root: TrieNode::new(),
        }
    }

    fn insert(&mut self, word: String) {
        let mut node = &mut self.root;
        for chr in word.chars() {
            if let None = node.children.get(&chr) {
                node.children.insert(chr, TrieNode::new());
            }
            node = node.children.get_mut(&chr).unwrap();
        }
        node.is_complete_word = true
    }

    fn search(&self, word: String) -> bool {
        let mut node = &self.root;
        for chr in word.chars() {
            if let None = node.children.get(&chr) {
                return false;
            }
            node = node.children.get(&chr).unwrap();
        }
        node.is_complete_word
    }

    fn starts_with(&self, prefix: String) -> bool {
        let mut node = &self.root;
        for chr in prefix.chars() {
            if let None = node.children.get(&chr) {
                return false;
            }
            node = node.children.get(&chr).unwrap();
        }
        true
    }
}