// https://leetcode.com/problems/maximum-sum-circular-subarray/solutions/3066663/rust-kadane-s-algorithm-enumerate-prefix-and-suffix-sums/
impl Solution {
    pub fn max_subarray_sum_circular(nums: Vec<i32>) -> i32 {
        i32::max(
            nums.iter()
                .fold((i32::MIN, 0), |(m, c), &n| match c + n {
                    c => (
                        i32::max(m, c),
                        match c < 0 {
                            true => 0,
                            false => c,
                        },
                    ),
                })
                .0,
            Iterator::zip(
                nums.iter()
                    .scan(0, |p, &n| Some(p.assign(*p + n)))
                    .scan(i32::MIN, |l, p| Some(l.assign(i32::max(*l, p)))),
                nums.iter()
                    .rev()
                    .scan(0, |s, &n| Some(s.assign(*s + n)))
                    .scan(i32::MIN, |r, s| Some(r.assign(i32::max(*r, s))))
                    .collect::<Vec<i32>>()
                    .into_iter()
                    .rev()
                    .skip(1)
                    .chain([0]),
            )
            .fold(i32::MIN, |m, (l, r)| i32::max(m, l + r)),
        )
    }
}

trait Monad {
    fn assign(&mut self, v: Self) -> Self;
}
impl Monad for i32 {
    fn assign(&mut self, v: Self) -> Self {
        *self = v;
        *self
    }
}