// https://leetcode.com/problems/climbing-stairs/solutions/3253817/rust-solution-recursion-and-dynamic-programming/
impl Solution {
    pub fn climb_stairs(n: i32) -> i32 {
        fn _climb_stairs(n: i32, cache: &mut Vec<i32>) -> i32 {
            let nusize = n as usize;
            match n {
                0 => {
                    cache[nusize] = 0;
                    return 1;
                }
                1 => {
                    cache[nusize] = 1;
                    return 1;
                }
                _ => match cache[nusize] != 0 {
                    true => cache[nusize],
                    false => {
                        let val = _climb_stairs(n - 1, cache) + _climb_stairs(n - 2, cache);
                        cache[nusize] = val;
                        return val;
                    }
                },
            }
        }

        let mut cache: Vec<i32> = vec![0; (n + 1) as usize];
        _climb_stairs(n, &mut cache)

    }
}