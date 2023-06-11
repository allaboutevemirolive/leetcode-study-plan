// https://leetcode.com/problems/plus-one/solutions/3291762/rust-using-vecdeque/
impl Solution {
    pub fn plus_one(digits: Vec<i32>) -> Vec<i32> {
        use std::collections::VecDeque;
        
        let mut result = VecDeque::new();
        let mut carry = 1;
        for (index, digit) in digits.iter().rev().enumerate() {
            let mut num = digit + carry;
            if num >= 10 {
                carry = 1;
                num = num % 10;
            } else {
                carry = 0;
            }
            result.push_front(num);
        }
        
        if carry == 1 {
            result.push_front(1);
        }

        result.into_iter().collect()
    }
}