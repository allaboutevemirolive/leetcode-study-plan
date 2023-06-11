// https://leetcode.com/problems/add-binary/solutions/3441334/rust-2ms-2mb/
impl Solution {
    pub fn add_binary(a: String, b: String) -> String {
        let (a, b) = if a.len() > b.len() { (a, b) } else { (b, a) };
        let to_digit = |n: char| n.to_digit(2).unwrap() as u8;
        let to_char = |d| match d {
            0 => '0',
            1 => '1',
            2 => '2',
            3 => '3',
            _ => unreachable!(),
        };
        let mut b = b.chars().rev();
        let mut carry = 0u8;
        let mut result = String::new();
        for a in a.chars().rev() {
            let a = to_digit(a);
            let b = b.next().map(to_digit).unwrap_or(0);
            let sum = a + b + carry;
            carry = if sum <= 1 { 0 } else { 1 };
            result.insert(0, to_char(sum % 2));
        }
        if carry == 1 {
            result.insert(0, '1');
        }
        result
    }
}