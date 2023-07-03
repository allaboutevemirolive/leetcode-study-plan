// https://leetcode.com/problems/simplify-path/solutions/3410222/almost-idiomatic-rust-solution/
impl Solution {
    pub fn simplify_path(path: String) -> String {
        let path: Vec<&str> = path.split('/').collect();
        let mut stack: Vec<&str> = Vec::new();
        
        for folder in path {
            match folder {
                "." | "" => {},
                ".." => { stack.pop(); },
                _ => { stack.push(folder); },
            }
        }

        format!("/{}", stack.join("/"))
    }
}