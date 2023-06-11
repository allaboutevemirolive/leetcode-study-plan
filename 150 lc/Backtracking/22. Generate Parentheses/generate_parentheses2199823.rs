// https://leetcode.com/problems/generate-parentheses/solutions/2199823/rust-clear-solution-no-unnecessary-clones/
impl Solution {
    pub fn generate_parenthesis(n: i32) -> Vec<String> {
        let mut answer = Vec::with_capacity(200);
        let mut string_buffer = String::with_capacity(100);

        Self::rec_impl_generate_parenthesis(
            (n * 2) as usize,
            n,
            n,
            &mut answer,
            &mut string_buffer,
        );

        answer
    }

    fn rec_impl_generate_parenthesis(
        string_target_size: usize,
        open_remaining: i32,
        close_remaining: i32,
        answer: &mut Vec<String>,
        string_buffer: &mut String,
    ) {
        if string_buffer.len() == string_target_size {
            answer.push(string_buffer.clone());
            return;
        }

        if open_remaining > 0 {
            string_buffer.push('(');

            Self::rec_impl_generate_parenthesis(
                string_target_size,
                open_remaining - 1,
                close_remaining,
                answer,
                string_buffer,
            );

            string_buffer.pop();
        }

        if close_remaining > open_remaining {
            string_buffer.push(')');

            Self::rec_impl_generate_parenthesis(
                string_target_size,
                open_remaining,
                close_remaining - 1,
                answer,
                string_buffer,
            );

            string_buffer.pop();
        }
    }
}