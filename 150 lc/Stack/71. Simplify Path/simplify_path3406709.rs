// https://leetcode.com/problems/simplify-path/solutions/3406709/rust-stack-with-pattern-matching/
impl Solution {
    pub fn simplify_path(path: String) -> String {
        let mut stack: Vec<&str> = Vec::new();
        let path = path + "/";
        let mut parts = path.split('/');
        
        while let Some(part) = parts.next() {
            match part {
                ".." => {
                    if !stack.is_empty() {
                        stack.pop();
                    }
                },
                "." | "" => {},
                _ => {
                    stack.push(part);
                },
            }
        }

        format!("/{}", stack.join("/"))
    }
}