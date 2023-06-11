// https://leetcode.com/problems/evaluate-reverse-polish-notation/solutions/2919649/rust-0ms-2-7-mb/
impl Solution {
    pub fn eval_rpn(tokens: Vec<String>) -> i32 {
        let mut vec = vec![];
        for chunk in tokens {
            let num = chunk.parse::<i32>();
            if num.is_ok() {
                vec.push(num.unwrap());
            } else {
                let b = vec.pop().unwrap();
                let a = vec.pop().unwrap();
                let c = match chunk.as_str() {
                    "+" => a + b,
                    "-" => a - b,
                    "*" => a * b,
                    "/" => a / b,
                    _   => unreachable!(),
                };
                vec.push(c);
            }
        }
        vec.pop().unwrap()
    }
}