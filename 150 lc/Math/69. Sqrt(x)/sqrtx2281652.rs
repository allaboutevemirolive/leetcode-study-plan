// https://leetcode.com/problems/sqrtx/solutions/2281652/rust-solution-with-tests/
struct Solution {}

impl Solution {
    pub fn my_sqrt(x: i32) -> i32 {
        use std::cmp::Ordering;

        let mut start = 0;
        let mut end = x;
        let mut answer = 0;

        while start <= end {
            let midlle = start + (end - start) / 2;

            match midlle.checked_mul(midlle) {
                Some(val) => match val.cmp(&x) {
                    Ordering::Less => {
                        answer = midlle;
                        start = midlle + 1;
                    }
                    Ordering::Equal => {
                        return midlle;
                    }
                    Ordering::Greater => {
                        end = midlle - 1;
                    }
                },
                None => {
                    end = midlle - 1;
                }
            }
        }

        answer
    }
}

mod tests {
    use crate::sqrt::Solution;

    #[test]
    fn test() {
        assert_eq!(2, Solution::my_sqrt(5));
        assert_eq!(2, Solution::my_sqrt(4));
        assert_eq!(1, Solution::my_sqrt(3));
        assert_eq!(1, Solution::my_sqrt(2));
        assert_eq!(1, Solution::my_sqrt(1));
        assert_eq!(0, Solution::my_sqrt(0));

        assert_eq!(2, Solution::my_sqrt(6));
        assert_eq!(2, Solution::my_sqrt(4));
        assert_eq!(2, Solution::my_sqrt(8));
    }
}