// https://leetcode.com/problems/simplify-path/solutions/3407694/rust-stack-clean-explained/
impl Solution {
    pub fn simplify_path(path: String) -> String {
        let mut cano = Vec::new();
        let mut components = path.split('/');

        for cmp in components {
            match cmp {
                ".." => if !cano.is_empty() {cano.pop();} else {continue;},
                "." | "" => continue,
                _ => cano.push(cmp)
            };
        }

        let mut res = cano.join("/");
        res.insert(0, '/');
        res
    }
}