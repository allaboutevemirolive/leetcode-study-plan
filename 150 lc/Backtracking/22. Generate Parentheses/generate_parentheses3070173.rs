// https://leetcode.com/problems/generate-parentheses/solutions/3070173/rust-simple-dfs-0ms-2-2mb/
impl Solution {
    pub fn generate_parenthesis(n: i32) -> Vec<String> {
        let mut answer = vec![];
        let mut parens = "".to_string();

        Self::dfs(n as usize, 0, 0, &mut parens, &mut answer);

        answer
    }

    fn dfs(n      : usize, 
           i      : usize, 
           n_open : usize, 
           parens : &mut String, 
           answer : &mut Vec<String>) 
    {
        let max_i = 2 * n - 1;

        if i < max_i - n_open {
            parens.push('(');
            Self::dfs(n, i + 1, n_open + 1, parens, answer);
            parens.pop().unwrap();
        }
        if n_open > 0 {
            parens.push(')');
            if i < max_i {
                Self::dfs(n, i + 1, n_open - 1, parens, answer);
            } else {
                answer.push(parens.clone());
            }
            parens.pop().unwrap();
        }
    }
}