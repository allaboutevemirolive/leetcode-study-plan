// https://leetcode.com/problems/generate-parentheses/solutions/3485252/my-rust-solution-beats-100/
use std::collections::HashSet;
impl Solution {
    pub fn generate_parenthesis(n: i32) -> Vec<String> {
        if n == 1 {
            return vec!("()".to_string());
        }
        let mut all: Vec<String> = Self::generate_parenthesis(n-1);
        for item in &mut all {
            item.insert(0, '(');
            item.push(')');
        }
        let mut set: HashSet<String> = HashSet::new();
        for i in 1..n {
            let left = Self::generate_parenthesis(i);
            let right = Self::generate_parenthesis(n-i);
            for li in &left {
                for ri in &right {
                    set.insert(li.to_owned() + ri);
                }
            }
        }
        all.extend(set.into_iter());
        return all;
    }
}
