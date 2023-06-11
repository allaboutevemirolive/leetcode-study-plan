// https://leetcode.com/problems/contains-duplicate-ii/solutions/2063774/rust-solution-with-hashmap-o-n/

impl Solution {
    pub fn contains_nearby_duplicate(nums: Vec<i32>, k: i32) -> bool 
    {
        let mut map:HashMap<i32,i32> = HashMap::new();   
        
        for i in 0..nums.len()
        {
            // if current number is already found in HashMap,
            // compare value of i-j and update value in HashMap
            if(map.contains_key(&nums[i]))
            {
                let j=map.get(&nums[i]).unwrap();
                let diff=i as i32 - j;
                if diff.abs() <= k
                {
                    return true;
                }
                map.insert(nums[i], i as i32);
            }
            
            //else insert value in HashMap
            else
            {
                map.insert(nums[i], i as i32);
            }
        }
        
        return false;
    }
}