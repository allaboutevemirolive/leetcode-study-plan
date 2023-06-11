```rust
impl Solution {
    // [7,1,5,3,6,4] => 7
    // [1,2,3,4,5] => 4
    pub fn max_profit(prices: Vec<i32>) -> i32 {
        let mut ret = 0;
        for i in 0..prices.len() - 1 {
            let delta = prices[i + 1] - prices[i];
            if delta > 0 {
                ret += delta
            }
        }
        ret
    }
}
```