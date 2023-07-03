// https://leetcode.com/problems/plus-one/solutions/3208191/rust-epi-solution/
impl Solution {
    pub fn plus_one(mut digits: Vec<i32>) -> Vec<i32> {
        
        let len = digits.len() ;
        digits[len -1] += 1;

        for i in (1..len).rev() {
            if digits[i] != 10 {break;}
            digits[i] = 0;
            digits[i-1] += 1;
        }

        if digits[0] == 10 {
            digits[0] = 1;
            digits.push(0);
        }
        digits
    }
}