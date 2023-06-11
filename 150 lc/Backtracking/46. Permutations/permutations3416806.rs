// https://leetcode.com/problems/permutations/solutions/3416806/rust-backtracking-without-allocating-intermediate-vectors/
fn backtrack(nums: &mut Vec<i32>, 
            idx: usize,
            ans: &mut Vec<Vec<i32>>) {
    if idx >= nums.len() {
        ans.push(nums.clone());
        return;
    }

    for i in idx..nums.len() {
        nums.swap(i, idx);
        backtrack(nums, idx + 1, ans);
        nums.swap(idx, i);
    }
}

impl Solution {
    pub fn permute(mut nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut ans = vec![];
        backtrack(&mut nums, 0, &mut ans);
        ans
    }
}