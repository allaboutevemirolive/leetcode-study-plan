// https://leetcode.com/problems/contains-duplicate-ii/solutions/1922679/rust-solution-using-hashmap/
pub fn contains_nearby_duplicate(nums: Vec<i32>, k: i32) -> bool {
    let mut map = std::collections::HashMap::new();
    for i in 0..nums.len() {
        // If key is present in map, return true
        let contains = map.contains_key(&nums[i]);
        if contains { return true; };

        // If not present, insert the key into map
        map.insert(nums[i], i);
        
        // If size of map becomes greater than k, then remove element from beginning side
        if map.len() > k as usize {
            map.remove(&nums[i - k as usize]);
        }
    }

    return false;
}