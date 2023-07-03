// https://leetcode.com/problems/simplify-path/solutions/3406701/rust-concise-solution-with-stack-pattern-matching-9-lines/
impl Solution {
    pub fn simplify_path(path: String) -> String {
        let mut stack = Vec::new();
        for dir in path.split("/") {
            match dir {
                "" | "." => (),
                ".." => { stack.pop(); },
                _ => stack.push(dir),
            }
        }
        format!("/{}", stack.join("/"))
    }
}
