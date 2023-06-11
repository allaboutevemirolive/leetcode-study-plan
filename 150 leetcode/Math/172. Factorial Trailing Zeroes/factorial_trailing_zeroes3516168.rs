// https://leetcode.com/problems/factorial-trailing-zeroes/solutions/3516168/rust-map-while-detailed-explanation/
impl Solution {
    pub fn trailing_zeroes(n: i32) -> i32 {
        (1..)
            .map_while(|power| match n / 5_i32.pow(power) {
                0 => None,
                m => Some(m),
            })
            .sum()
    }
}