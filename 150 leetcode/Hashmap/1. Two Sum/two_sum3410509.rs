// https://leetcode.com/problems/two-sum/solutions/3410509/rust-solution-that-beats-100-in-runtime/
use::std::collections::HashMap;

impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut elements : Vec<Option<&i32>> = Vec::new();
        let els : Vec<i32> = vec![1,2,3];
        let mut hm: HashMap<i32, i32> = HashMap::new();
        for i in 0..nums.len() {
            
            if !hm.contains_key(&(target-nums[i])){
                hm.insert(nums[i], i as i32);
            }
            else {
                let copy_i = i.clone() as i32;
                elements.push(hm.get(&(target-nums[i])));
                elements.push(Some(&(copy_i)));
                return elements.into_iter()
                            .filter_map(|x| x.map(|y| *y))
                            .collect();
            }
        }
        return els;
    }
}