// https://leetcode.com/problems/plus-one/solutions/2959508/rust-solution/
impl Solution {
    pub fn plus_one(digits: Vec<i32>) -> Vec<i32> {
        let mut digits = digits;
        let mut is_carry_over = false;

        let length = digits.len();
        digits[length - 1] = digits[length - 1] + 1;

        if digits[length - 1] % 10 == 0 {
            digits[length - 1] = 0;
            is_carry_over = true;
        }

        for i in (0..digits.len() - 1).rev() {
            if is_carry_over {
                let new_value = digits[i] + 1;

                if new_value % 10 == 0 {
                    digits[i] = 0
                } else {
                    digits[i] = new_value;
                    is_carry_over = false;
                }
            }
        }

        if is_carry_over {
            digits.insert(0, 1);
        }

        digits
    }
}