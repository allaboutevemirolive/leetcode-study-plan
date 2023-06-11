// https://leetcode.com/problems/sqrtx/solutions/2528245/rust-binary-search-solution/
impl Solution {
    pub fn my_sqrt(x: i32) -> i32 {
        let (mut low, mut high) = (1, x);
        while low <= high {
			// let mid = ((high + low + 1) >> 1); // Caution: overflowed if high = i32::MAX
            let mid = low + ((high - low + 1) >> 1);
            match mid - x / mid {
                0 => return mid,
                1.. => high = mid - 1,
                _ => low = mid + 1,
            }
        }
        return low - 1;
    }
}