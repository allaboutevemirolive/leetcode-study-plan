// https://leetcode.com/problems/add-binary/solutions/3183441/rust/
impl Solution {
    pub fn add_binary(a: String, b: String) -> String {
        let mut a = a.bytes().collect::<Vec<_>>();
        let mut b = b.bytes().collect::<Vec<_>>();
        a.reverse();
        b.reverse();
        let (m, n, mut i, mut j) = (a.len(), b.len(), 0, 0);
        let mut ans: Vec<u8> = vec![];
        let mut sig = 0;
        loop {
            if i == m && j == n {
                break;
            }
            let mut ta = 0;
            if i < m {
                ta = a[i]-b'0';
                i += 1;
            }
            let mut tb = 0;
            if j < n {
                tb = b[j]-b'0';
                j += 1;
            }
            let mut su = ta + tb + sig;
            if su > 1 {
                su -= 2;
                sig = 1;
            } else {
                sig = 0;
            }
            ans.push(su+b'0')
        }
        if sig > 0 {
            ans.push(b'1')
        }

        ans.reverse();
        String::from_utf8(ans).unwrap()
    }
}