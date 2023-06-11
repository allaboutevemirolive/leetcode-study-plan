// https://leetcode.com/problems/simplify-path/solutions/3406723/very-simple-and-concise-solution-with-explanation/
impl Solution {
    pub fn simplify_path(path: String) -> String {
        let mut simplified_path = vec![];
        for dir in path.split('/') {
            match dir {
                "" | "." => continue,
                ".." => { simplified_path.pop(); }
                _ => simplified_path.push(dir),
            }
        }

        "/".to_owned() + &simplified_path.join("/")
    }
}