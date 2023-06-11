// https://leetcode.com/problems/add-binary/solutions/3184247/easy-1-line-0ms-rust-solution-beats-100/
impl Solution {
    pub fn add_binary(a: String, b: String) -> String {
        format!("{:b}", i128::from_str_radix(&*b, 2).unwrap() + i128::from_str_radix(&*a, 2).unwrap())
    }
}