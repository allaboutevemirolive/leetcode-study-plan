// https://leetcode.com/problems/add-binary/solutions/3184435/20-lines-iterators-rust/
impl Solution {
    pub fn add_binary(a: String, b: String) -> String {
        let mut carry = 0;

        let f = |(a, b)| {
            let sum = a - b'0' + b - b'0' + carry;
            carry = sum / 2;
            sum % 2 + b'0'
        };
        let v1: Vec<_> = a.bytes().rev().zip(b.bytes().rev()).map(f).collect();

        let f = |a| {
            let sum = a - b'0' + carry;
            carry = sum / 2;
            sum % 2 + b'0'
        };
        let longest = if a.len() > b.len() { &a } else { &b };
        let n_short = if a.len() > b.len() { b.len() } else { a.len() };
        let v2: Vec<_> = longest.bytes().rev().skip(n_short).map(f).collect();

        let carry = Some(carry + b'0').filter(|&c| c > b'0');
        let v = v1.into_iter().chain(v2).chain(carry).rev().collect();
        String::from_utf8(v).unwrap()
    }
}
