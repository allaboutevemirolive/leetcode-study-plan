// https://leetcode.com/problems/simplify-path/solutions/3408918/rust-100-faster-stack-simple-and-easy/
impl Solution {
    pub fn simplify_path(path: String) -> String {
        let len = path.len();
        let mut res = String::with_capacity(len);
        let mut stack = Vec::with_capacity(1000);
        for i in path.split('/') {
            match i {
                ".." => {
                    stack.pop();
                },
                "." | "" => {
                    continue;
                }
                _ => {
                    stack.push(i);
                }
            }
        }
        for i in stack {
            if !i.is_empty() {
                res.push('/');
                res.push_str(i);
            }
        }
        if res.is_empty() {
            res.push('/');
        }
        res
    }
}
