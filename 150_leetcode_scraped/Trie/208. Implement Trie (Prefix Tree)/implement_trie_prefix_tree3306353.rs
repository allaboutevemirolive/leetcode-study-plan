// https://leetcode.com/problems/implement-trie-prefix-tree/solutions/3306353/concise-no-code-duplication-solution-with-hashmap-in-rust/
use std::collections::HashMap;

struct TrieNode {
    is_word: bool,
    children: HashMap<u8, TrieNode>
}

impl TrieNode {
    fn new() -> Self {
        TrieNode { is_word: false, children: HashMap::new() }
    }
}

struct Trie {
    root: TrieNode
}

impl Trie {

    fn new() -> Self {
        Trie { root: TrieNode::new() }
    }
    
    fn insert(&mut self, word: String) {
        let mut curr_node = &mut self.root;
        for c in word.bytes() {
            curr_node = curr_node.children.entry(c).or_insert_with(TrieNode::new);
        }
        curr_node.is_word = true;
    }

    fn search_node(&self, prefix: String) -> Option<&TrieNode> {
        let mut curr_node = Some(&self.root);
        for c in prefix.bytes() {
            curr_node = if let Some(curr_node) = curr_node { curr_node.children.get(&c) } else { break };
        }
        curr_node
    }
    
    fn search(&self, word: String) -> bool {
        self.search_node(word).map_or(false, |node| node.is_word)
    }
    
    fn starts_with(&self, prefix: String) -> bool {
        self.search_node(prefix).is_some()
    }
}