// https://leetcode.com/problems/simplify-path/solutions/3408813/rust-1-line-and-for-loop-solutions-0ms-beats-100/
impl Solution {
    pub fn simplify_path(path: String) -> String {
        let mut dirs = vec![];
        for token in path.split('/') {
            match token {
                "." | "" => continue,
                ".." => {
                    dirs.pop();
                }
                _ => dirs.push(token),
            }
        }
        format!("{}{}", "/", dirs.join("/"))
    }
}
