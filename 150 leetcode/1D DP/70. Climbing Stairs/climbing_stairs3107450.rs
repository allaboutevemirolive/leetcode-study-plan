// https://leetcode.com/problems/climbing-stairs/solutions/3107450/easy-solution-in-rust-that-uses-addition-of-only-two-variables-and-a-loop-nothing-fancy/
impl Solution {
    pub fn climb_stairs(n: i32) -> i32 {
        if n== 1 || n == 2 || n == 3{
            return n;
        }
        let mut previous : i32 = 2;
        let mut current : i32 = 3;
        let mut count : i32 = 3;
        // we have will have three variables previous, current and result
        // first we add previous and current to set result's value
        // then we set the value of previous to current's value
        // then we set current's value to result's value, and at the end of the
        // loop we return current
        while count < n{
            let result : i32 = previous + current;
            previous = current;
            current = result;
            count += 1;
        }
        current
    }
}