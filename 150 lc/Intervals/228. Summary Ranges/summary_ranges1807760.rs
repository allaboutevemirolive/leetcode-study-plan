// https://leetcode.com/problems/summary-ranges/solutions/1807760/rust-1ms-2-1-mb/
// https://leetcode.com/problems/summary-ranges/discuss/1805391/Concise-Solution-in-0(N)-with-approach-explained-in-detail
impl Solution {
    pub fn summary_ranges(nums: Vec<i32>) -> Vec<String> {
        let mut res: Vec<String> = Vec::new();
        // the counter will allow us to keep track of our traversal 
        let mut i = 0;
        
        while i < nums.len() { 
            let start = nums[i];
            //  Check if i has reached the end and that the ith element added with 1
            //  gives us the next element
            while i + 1< nums.len()  && nums[i + 1] == nums[i] + 1 { 
                i +=1
            }
			// turn the stored val into string
            let mut into_string = start.to_string();
            //  As soon as we find a point where nums[i] +1 != nums[i + 1] 
            //  we check if the stored start val != the current num
            if start != nums[i] { 
                //  add the string to the list of strings we are creating
                into_string += "->";
                into_string += &nums[i].to_string()
                //  so that we return a string 
                //  "start" "->" "nums[i]"
            };
            //  Add the resulting string to our string 
            res.push(into_string);
            i +=1
        }
        res
    }
}