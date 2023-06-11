// https://leetcode.com/problems/two-sum/solutions/3586578/rust-hashmap/
use std::collections::HashMap;
impl Solution 
{
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> 
    {
        let mut hm1 :HashMap<i32, i32> = HashMap::with_capacity(nums.capacity());
        let result :Vec<i32> = Vec::new();
        for (index, &value) in nums.iter().enumerate()  
        {
            let x = target - value;
            if hm1.contains_key(&x)
            {
                let index_2 = hm1[&x];
                return vec![index_2, index as i32];
            }
            else 
            {
                hm1.insert(value, index as i32);               
            } 
        }
        result      
    }
}