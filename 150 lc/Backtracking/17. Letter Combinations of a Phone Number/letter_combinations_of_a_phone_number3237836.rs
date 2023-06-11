// https://leetcode.com/problems/letter-combinations-of-a-phone-number/solutions/3237836/my-efficient-rust-solutiion/
impl Solution {
    pub fn letter_combinations(digits: String) -> Vec<String> {
        if digits.is_empty() {
            return vec![]
        }
        let nums = Vec::from(["", "", "abc", "def", "ghi", "jkl", "mno", "pqrs", "tuv", "wxyz"]);
        digits.bytes().fold(vec!["".to_string()],| mut init, vals| {
            let mut new_vec = Vec::with_capacity(init.len()*3);
            let letters = nums[(vals - b'0') as usize].chars().collect::<Vec<_>>();
            while let Some(val) = init.pop() {
                letters.iter().for_each(|leter|{
                    new_vec.push(format!("{}{}", val,leter));
                })
            }
            new_vec
        })
    }
}