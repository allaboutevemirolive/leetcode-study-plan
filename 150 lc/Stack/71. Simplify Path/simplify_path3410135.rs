// https://leetcode.com/problems/simplify-path/solutions/3410135/rust-solution-stack/
impl Solution {
    pub fn simplify_path(path: String) -> String {
        let mut stack:Vec<String> = vec![];

        let mut parts = path.split('/');
        for part in parts {
            match part {
                "." | "" => {
                    continue;
                }
                ".." => {
                    if !stack.is_empty() {
                        stack.pop();
                    }
                }
                _ => {
                    stack.push(part.to_string());
                }
            }
        }

        let mut res = String::new();

        for part in stack {
            res.push_str("/");
            res.push_str(&part);
        }
        
        if res.is_empty() {
            return String::from("/");
        }
        res
    }
}