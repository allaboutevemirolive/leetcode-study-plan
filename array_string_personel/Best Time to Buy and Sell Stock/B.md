```rust
impl Solution {
    // [7,1,5,3,6,4]
    
    pub fn max_profit(prices: Vec<i32>) -> i32 {
        let mut result = 0;
        let mut best_buy_price = i32::MAX;
        
        for price in prices {
            best_buy_price = best_buy_price.min(price);
            result = result.max(price - best_buy_price);
        }
        
        result
    }
}
```