// https://leetcode.com/problems/simplify-path/solutions/3407118/rust-simple-stack/
impl Solution {
    pub fn simplify_path(path: String) -> String {
        let mut s = Vec::new();    
        for tok in path.split("/") {
            match tok {
                "" | "." => continue,    
                ".."     => { s.pop(); }
                e        => { s.push(tok); }
            }
        } 
        return format!("/{}", s.join("/")); 
    }
}