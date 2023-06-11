// https://leetcode.com/problems/two-sum/solutions/3605384/my-solution-using-rust/
impl Solution 
{
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> 
    {
        let mut v: Vec<i32> = Vec::new();
        
        for i in 0..nums.len() 
        {
            for n in i + 1..nums.len() 
            {
                if nums[i] + nums[n] == target 
                {
                    v.push(i as i32);
                    v.push(n as i32);
                    return v;
                }
            }
        }
        v // Return an empty vector if no solution is found
    }
}
