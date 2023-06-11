// https://leetcode.com/problems/climbing-stairs/solutions/3274352/rust-dp-with-less-mem-cost/
impl Solution {
    pub fn climb_stairs(n: i32) -> i32 {
        if n == 1 {
            return 1;
        }
        if n == 2 {
            return 2;
        }
        let mut two_stairs_before = 1;
        let mut one_stairs_before = 2;
        let mut num_of_ways = 3;
        let mut curr_stair = 3;
        while curr_stair < n {
            two_stairs_before = one_stairs_before;
            one_stairs_before = num_of_ways;
            num_of_ways = one_stairs_before + two_stairs_before;
            curr_stair += 1;
        }
        num_of_ways
    }
}