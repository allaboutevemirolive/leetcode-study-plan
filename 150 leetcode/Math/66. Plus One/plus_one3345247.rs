// https://leetcode.com/problems/plus-one/solutions/3345247/solution-for-rust/
impl Solution {
    pub fn plus_one(mut digits: Vec<i32>) -> Vec<i32> {
        let mut idx = (digits.len()-1) as i32;
        let mut carry = 0;

        while idx >= 0 {
            carry = 0;
            if let Some(val) = digits.get_mut(idx as usize) {
                *val = (*val + 1) % 10;
                if *val == 0 {
                    carry = 1;
                }
            }

            if carry == 1 {
                idx -= 1;
            } else {
                break;
            }
        }

        if carry == 1 {
            digits.insert(0, 1);
        }

        digits
    }
}