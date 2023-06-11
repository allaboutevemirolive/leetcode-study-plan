// https://leetcode.com/problems/add-binary/solutions/3557348/rust-mostly-iterators-easy-to-understand-process-each-pair-of-chars-at-the-time/
impl Solution {
    pub fn add_binary(a: String, b: String) -> String {
        let max_size = std::cmp::max(a.len(), b.len());
        let a = format!("{:0>width$}", a, width=max_size);
        let b = format!("{:0>width$}", b, width=max_size);

        let mut res = a.chars().rev().zip(
                b.chars().rev()
            ).fold((0, String::new()), |mut acc, (x,y)| {
                let (carry, result) = acc;
                let x = x.to_digit(10).unwrap();
                let y = y.to_digit(10).unwrap();
                match (carry + x + y) {
                    0 => (0, result + "0"),
                    1 => (0, result + "1"),
                    2 => (1, result + "0"),
                    _ => (1, result + "1"),
                }
            });
            if res.0 > 0 {
                res.1.push_str("1");
            }
            res.1.chars().rev().collect::<String>()
    }
}