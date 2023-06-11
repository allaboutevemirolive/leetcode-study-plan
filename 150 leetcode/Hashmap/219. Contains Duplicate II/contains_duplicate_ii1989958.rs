// https://leetcode.com/problems/contains-duplicate-ii/solutions/1989958/rust-solution/
impl Solution {
    pub fn contains_nearby_duplicate(nums: Vec<i32>, k: i32) -> bool {
        let mut map = std::collections::HashMap::new();
        for (&num, i) in nums.iter().zip(0..) {
            let e = map.entry(num).or_insert(-1);
            match *e != -1 && i - *e <= k {
                true => return true,
                false => *e = i,
            }
        }
        false
    }
}