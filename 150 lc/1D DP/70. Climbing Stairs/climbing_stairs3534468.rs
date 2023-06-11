// https://leetcode.com/problems/climbing-stairs/solutions/3534468/rust-compile-time-solution/
const fn solve() -> [i32; 46] {
    let mut nums: [i32; 46] = [0; 46];
    nums[0] = 1;
    nums[1] = 1;

    let mut i = 2;
    while i < 46 {
        nums[i] = nums[i - 1] + nums[i - 2];
        i += 1;
    }
    nums
}

const fibs: [i32; 46] = solve();

impl Solution {
    pub fn climb_stairs(n: i32) -> i32 {
        fibs[n as usize]
    }
}
