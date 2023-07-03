// https://leetcode.com/problems/generate-parentheses/solutions/3418616/rust-recursion-of-parenthesis-combinations/
impl Solution {
    pub fn generate_parenthesis(n: i32) -> Vec<String> {
        let mut result = vec![];
        let mut current = String::new();

        fn generate_helper(result: &mut Vec<String>, current: &mut String, left: i32, right: i32) {
            if left == 0 && right == 0 {
                result.push(current.clone());
                return;
            }

            if left > 0 {
                current.push('(');
                generate_helper(result, current, left - 1, right);
                current.pop();
            }

            if right > left {
                current.push(')');
                generate_helper(result, current, left, right - 1);
                current.pop();
            }
        }

        generate_helper(&mut result, &mut current, n, n);
        result
    }
}