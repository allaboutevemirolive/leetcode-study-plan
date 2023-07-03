// https://leetcode.com/problems/implement-trie-prefix-tree/solutions/3306014/rust-detailed-explanation-and-solution/
struct TrieNode {
    children: [Option<Box<TrieNode>>; 26],
    is_end_of_word: bool,
}

impl TrieNode {
    fn new() -> Self {
        TrieNode {
            children: Default::default(),
            is_end_of_word: false,
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
        let mut current = &mut self.root;
        for c in word.chars() {
            let index = (c as usize) - ('a' as usize);
            current = current.children[index].get_or_insert_with(|| Box::new(TrieNode::new()));
        }
        current.is_end_of_word = true;
    }
    
    fn search(&self, word: String) -> bool {
        let mut current = &self.root;
        for c in word.chars() {
            let index = (c as usize) - ('a' as usize);
            match &current.children[index] {
                Some(child) => current = child,
                None => return false,
            }
        }
        current.is_end_of_word
    }
    
    fn starts_with(&self, prefix: String) -> bool {
        let mut current = &self.root;
        for c in prefix.chars() {
            let index = (c as usize) - ('a' as usize);
            match &current.children[index] {
                Some(child) => current = child,
                None => return false,
            }
        }
        true
    }
}