// https://leetcode.com/problems/plus-one/solutions/3191057/list-comprehensions-and-match-rust/
impl Solution {
    pub fn plus_one(digits: Vec<i32>) -> Vec<i32> {
        let mut answer:Vec<i32> = Vec::new();
        let mut carry_digit = 1;
        for d in digits.into_iter().rev() {
            match (carry_digit, d){
                (1,9) => answer.push(0),
                (0,_) => answer.push(d),
                (1,_) => { 
                    answer.push(d + 1);
                    carry_digit = 0;
                },
                (_,_) => println!("the world has ended"),
            }
        }
        if carry_digit == 1 {
            answer.push(1);
        }
        answer.into_iter().rev().collect()
    }
}