// https://leetcode.com/problems/add-binary/solutions/3186088/rust-simple-solution/
impl Solution {
    pub fn add_binary(a: String, b: String) -> String {
        let a = a.chars().rev().collect::<String>();
        let b = b.chars().rev().collect::<String>();
        //println!("{}, {}", &a, &b);
        let mut carry = 0;
        let mut res = String::from("");
        let na = a.len();
        let nb = b.len();
        for i in 0..std::cmp::max(na, nb) {
            let ca = a.chars().nth(i).unwrap_or('0').to_digit(2).unwrap();
            let cb = b.chars().nth(i).unwrap_or('0').to_digit(2).unwrap();
            match ca + cb + carry {
                0 => {
                    carry = 0;
                    res = res + "0";
                },
                1 => {
                    carry = 0;
                    res = res + "1";
                },
                2 => {
                    carry = 1;
                    res = res + "0";
                },
                3 => {
                    carry = 1;
                    res = res + "1";
                },
                _ => panic!("invalid addition"),
            }
        }
        if carry > 0 {
            res = res + "1";
        }
        return res.chars().rev().collect::<String>();
    }
}