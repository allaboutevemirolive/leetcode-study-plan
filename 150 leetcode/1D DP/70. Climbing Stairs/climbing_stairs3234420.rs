// https://leetcode.com/problems/climbing-stairs/solutions/3234420/rust-solution-with-memoization-0ms-runtime/
use std::collections::HashMap;

impl Solution {
    pub fn climb_stairs(n: i32) -> i32 {
        let mut memo = HashMap::new();
        pub fn climber(n: i32, memo: &mut HashMap<i32, i32>) -> i32 {
            match n {
                1 => { return 1; },
                2 => { return 2; },
                _ => {
                    match memo.get(&n) {
                        Some(value) => return *value,
                        None => {
                            let value = climber(n.clone() - 1, memo) + climber(n.clone() - 2, memo);
                            memo.insert(n, value);
                            return value;
                        }
                    }
                }
            }
        };

        climber(n, &mut memo)
    }
}