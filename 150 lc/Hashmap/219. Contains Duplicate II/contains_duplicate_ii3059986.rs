// https://leetcode.com/problems/contains-duplicate-ii/solutions/3059986/go-rust-fast-simple-solution-complexity-o-n/
RUST
impl Solution {
    pub fn contains_nearby_duplicate(nums: Vec<i32>, k: i32) -> bool {
        use std::collections::HashMap;
        let mut window: HashMap<i32,usize> = HashMap::new();
        let mut l = 0;
        for r in 0..nums.len() {
            if r - l > k as usize {
                window.remove(&nums[l]);
                l += 1;
            }
            match window.get(&nums[r]) {
                Some(_) => return true,
                _ => {window.insert(nums[r],1); },
            }
         }
         false
    }
}

GOLANG
func containsNearbyDuplicate(nums []int, k int) bool {
	window := map[int]int{}; 
    l := 0;
    for r := 0; r < len(nums); r++ {
        if r - l > k {
            delete(window, nums[l]);
            l +=1;
        }
        if _,ok := window[nums[r]]; ok {
            return true;
        }
        window[nums[r]] = 1;
    } 
    return false
}