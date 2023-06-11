// https://leetcode.com/problems/reverse-bits/solutions/2302714/rust-solution-0-ms/
impl Solution {
    pub fn reverse_bits(x: u32) -> u32 {
        let mut s = format!("{:b}", x);
        let padding = 32 - s.len();
        let mut pad = String::from("");
        
        for _ in 0..padding {
            pad.push('0');
        }
        
        s = format!("{}{}", pad, s);
        
        let mut result: u32 = 0;
        let mut running: u32 = 1;

        for c in s.chars() {
            match c {
                '1' => {
                    result += running;
                },
                _ => {}
            }

            if let Some(x) = running.checked_mul(2) {
                running = x;
            }
        }

        result
    }
}