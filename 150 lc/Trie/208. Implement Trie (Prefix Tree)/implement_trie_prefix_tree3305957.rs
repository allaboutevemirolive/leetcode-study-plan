// https://leetcode.com/problems/implement-trie-prefix-tree/solutions/3305957/rust-elixir-trie-solution-not-trie-solution/

#[derive(Default)]
struct Trie {
    flag: bool,
    next: [Option<Box<Trie>>; 26],
}

impl Trie {

    fn new() -> Self {
        Default::default()
    }
    
    fn insert(&mut self, word: String) {
        self.insert_node(&mut word.bytes());
    }

    fn insert_node(&mut self, bytes: &mut dyn Iterator<Item = u8>) {
        match bytes.next() {
            None => self.flag = true,
            Some(b) => {
                let i = (b - b'a') as usize;
                if self.next[i].is_none() {
                    self.next[i] = Some(Box::new(Trie::new()));
                }
                self.next[i].as_mut().unwrap().insert_node(bytes);
            }
        }
    }
    
    fn search(&self, word: String) -> bool {
        self.search_node(&mut word.bytes(), false)
    }
    
    fn starts_with(&self, prefix: String) -> bool {
        self.search_node(&mut prefix.bytes(), true)
    }

    fn search_node(&self, bytes: &mut dyn Iterator<Item = u8>, for_prefix: bool) -> bool {
        match bytes.next() {
            None => for_prefix || self.flag,
            Some(b) => {
                let i = (b - b'a') as usize;
                match self.next[i].as_ref() {
                    None => false,
                    Some(node) => node.search_node(bytes, for_prefix),
                }
            }
        }
    }
}