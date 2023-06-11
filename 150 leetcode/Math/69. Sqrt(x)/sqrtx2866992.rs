// https://leetcode.com/problems/sqrtx/solutions/2866992/rust-solution-without-overflow-checks-100/
use std::cmp::Ordering;

impl Solution {
    pub fn my_sqrt(x: i32) -> i32 {
        if x == 0 { // non-negative => 0 is allowed
            return 0;
        }

        let mut lower = 2;
        let mut higher = 46340.min(x >> 1); // max possible root sqrt(i32::MAX)

        // Boundary conditions for optimization

        if x <= 3 {
            return 1;
        }

        if x >= higher * higher {
            return higher;
        }

        // Binary search the correct perfect square

        while higher - lower > 1 {
            let mid = (higher + lower) >> 1; // same as (a+b)/2
            let pow = mid * mid;
            match pow.cmp(&x) {
                Ordering::Less => {
                    lower = mid;
                }
                Ordering::Greater => {
                    higher = mid;
                }
                _ => {
                    return mid;
                }
            }
        }

        // In the edge case that the number is between 2 values,
        // we take the lower which is equivalent to taking the floored mean
        lower
    }
}