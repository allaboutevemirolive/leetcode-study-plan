// https://leetcode.com/problems/climbing-stairs/solutions/3042812/matrix-exponentiation-solution-in-rust/
impl Solution {
    pub fn climb_stairs(n: i32) -> i32 {
        let mul = |a: &((i32, i32), (i32, i32)), b: &((i32, i32), (i32, i32))| (
            (a.0.0*b.0.0 + a.0.1*b.1.0, a.0.0*b.0.1 + a.0.1*b.1.1),
            (a.1.0*b.0.0 + a.1.1*b.1.0, a.1.0*b.0.1 + a.1.1*b.1.1));
        let (mut r, mut m) = (((1, 0), (0, 1)), ((1, 1), (1, 0)));
        for n in std::iter::successors(Some(n), |&n| if n > 0 { Some(n >> 1) } else { None }) {
            if n & 1 > 0 {
                r = mul(&r, &m);
            }
            m = mul(&m, &m);
        }
        r.0.0
    }
}