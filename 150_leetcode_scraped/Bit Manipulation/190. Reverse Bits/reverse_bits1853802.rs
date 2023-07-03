// https://leetcode.com/problems/reverse-bits/solutions/1853802/rust-using-low-bit-o-k-k-is-number-of-1-bit-in-x/
    pub fn reverse_bits(x: u32) -> u32 {
        let low_bit = |x: u64| (x as i64 & -(x as i64)) as u64;

        let mut x = x as u64;
        let mut hi = 1;
        let mut ans = 1;

        while x > 0 {
            let low = low_bit(x);
            hi *= low<<1;
            ans = ans*(low<<1) + 1;
            x /= (low<<1)
        }

        let mul = (1 << 32) / hi;
        (ans * mul) as u32
    }