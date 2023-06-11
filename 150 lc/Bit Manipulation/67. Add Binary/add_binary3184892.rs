// https://leetcode.com/problems/add-binary/solutions/3184892/rust-100-faster-imperetive-solution/
impl Solution {
    pub fn add_binary(mut a: String,mut b: String) -> String {
        let alen = a.len();
        let blen = b.len();
        if alen < blen  {
            std::mem::swap(&mut a, &mut b);
        }
        let a = a.chars().rev().collect::<Vec<char>>();
        let b = b.chars().rev().collect::<Vec<char>>();
        let alen = a.len();
        let blen = b.len();
        let mut v = vec!['0'; alen + 1];
        let mut carry = 0;
        let mut c = 0;
        for i in 0..blen {
            let sum = a[i] as u8 - '0' as u8 + b[i] as u8 - '0' as u8 + carry;
            if sum == 0 {
                v[i] = '0';
                carry = 0;
            } else if sum == 1 {
                v[i] = '1';
                carry = 0;
            } else if sum == 2 {
                v[i] = '0';
                carry = 1;
            } else {
                v[i] = '1';
                carry = 1;
            }
            c += 1;
        }
        for i in c..alen {
            let sum = a[i] as u8 - '0' as u8 + carry;
            if sum == 0 {
                v[i] = '0';
                carry = 0;
            } else if sum == 1 {
                v[i] = '1';
                carry = 0;
            } else {
                v[i] = '0';
                carry = 1;
            }   
        }
        v.reverse();
        if carry == 1 {
            v[0] = '1';
        }

        if v[0] == '0' {
            v[1..].iter().fold(String::with_capacity(alen), |mut acc, x| {acc.push(*x); acc})
        } else {
            v.iter().fold(String::with_capacity(alen), |mut acc, x| {acc.push(*x); acc})
        }
    }
}