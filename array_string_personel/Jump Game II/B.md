```rust
impl Solution {
    // [2,3,1,1,4]
    // [2,3,0,1,4]
    pub fn jump(mut nums: Vec<i32>) -> i32 {
        for i in 1..nums.len() {
            // println!(" i = {} nums[i] + i = {} nums[i - 1] = {}", i, nums[i] + i as i32, nums[i - 1]);
            // println!("nums[i] = {}", nums[i]);
            nums[i] = i32::max(nums[i] + i as i32, nums[i - 1]);
            // println!("nums[i] = {}", nums[i]);
        }
        // 2 _ 4 4 4 8
        // println!("{:?}", nums);
        // println!("nums.len = {}", nums.len());

        // [2,3,1,1,4]

        // i = 1 nums[i] + i = 4 nums[i - 1] = 2
        // nums[i] = 3
        // nums[i] = 4
        // i = 2 nums[i] + i = 3 nums[i - 1] = 4
        // nums[i] = 1
        // nums[i] = 4
        // i = 3 nums[i] + i = 4 nums[i - 1] = 4
        // nums[i] = 1
        // nums[i] = 4
        // i = 4 nums[i] + i = 8 nums[i - 1] = 4
        // nums[i] = 4
        // nums[i] = 8

        let mut ind = 0;
        let mut ans = 0;

        while ind < nums.len() - 1 {
            ans += 1;
            ind = nums[ind] as usize;

            println!("ans = {} ind = {}", ans, ind);
        }

        // [2, 4, 4, 4, 8]
        // nums.len = 5
        // ans = 1 ind = 2
        // ans = 2 ind = 4               

        return ans;
    }
}
```