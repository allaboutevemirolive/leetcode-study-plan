// https://leetcode.com/problems/evaluate-reverse-polish-notation/solutions/2921551/rust-stack-solution/
impl Solution {
    pub fn eval_rpn(tokens: Vec<String>) -> i32 {
        let mut stack = Vec::new();
        for t in tokens {
            if let Ok(n) = t.parse::<i32>() {
                stack.push(n);
            } else {
                if t.contains("+") {
                    let n1 = stack.pop().unwrap(); let n2 = stack.pop().unwrap();
                    stack.push(n2 + n1);
                } else if t.contains("-") {
                    let n1 = stack.pop().unwrap(); let n2 = stack.pop().unwrap();
                    stack.push(n2 - n1);
                } else if t.contains("*") {
                    let n1 = stack.pop().unwrap(); let n2 = stack.pop().unwrap();
                    stack.push(n2 * n1);
                } else if t.contains("/") {
                    let n1 = stack.pop().unwrap(); let n2 = stack.pop().unwrap();
                    stack.push(n2 / n1);
                } else {
                    panic!("unhandled token");
                }
            }
        }

        stack.pop().unwrap()
    }
}