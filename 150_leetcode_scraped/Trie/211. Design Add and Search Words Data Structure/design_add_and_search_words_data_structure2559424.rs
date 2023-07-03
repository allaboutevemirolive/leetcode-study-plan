// https://leetcode.com/problems/design-add-and-search-words-data-structure/solutions/2559424/rust-idiomatic-trie-solution-without-using-hashmap/
use std::collections::HashMap;

#[derive(Clone, Debug, Default, PartialEq)]
struct Node {
    children: Vec<Option<Node>>,
    is_word: bool,
}

impl Node {
    fn new() -> Self {
        Self {
            children: vec![None; 26],
            is_word: false
        }
    }
}
struct WordDictionary {
    root: Node,
}


/** 
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl WordDictionary {

    fn new() -> Self {
        Self {
            root: Node::new(),
        }        
    }
    
    fn add_word(&mut self, word: String) {
        let mut curr = &mut self.root;
        
        for b in word.bytes() {
            let idx = (b - b'a') as usize;
            
            if curr.children[idx] == None {
                curr.children[idx] = Some(Node::new());
            }
            curr = curr.children[idx].as_mut().unwrap();
        }
        curr.is_word = true;
    }
    
    fn search(&self, word: String) -> bool {
        let word = word.as_bytes();
        return Self::dfs(word, &self.root);
    }
    
    fn dfs(word: &[u8], mut node: &Node) -> bool {
        for idx in 0..word.len() {
             if word[idx] != b'.' {
                 if let Some(n) = node.children[(word[idx] - b'a') as usize].as_ref() {
                     node = n;
                 } else {
                     return false;
                 }
            } else {
                return node.children.iter().filter(|&v| v.is_some()).find(|child| {
                    Self::dfs(&word[idx + 1..], child.as_ref().unwrap())
                }).is_some();
            }
        }
        node.is_word
    }
}