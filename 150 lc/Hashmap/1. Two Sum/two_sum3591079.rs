// https://leetcode.com/problems/two-sum/solutions/3591079/rust-bin-search/
impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let sz = nums.len();
        let mut sorted = nums.into_iter().zip(0..sz).collect::<Vec<(i32, usize)>>();
        sorted.sort();
        for (starting_idx, (num, idx)) in sorted.iter().enumerate() {
            let seek = target - num;
            let res = sorted[starting_idx + 1..].binary_search_by(|(probe, _)| probe.cmp(&seek));
            match res {
                Ok(sorted_idx) => {
                    return vec![*idx as i32, (sorted[starting_idx + 1 + sorted_idx].1) as i32];
                }
                _ => {}
            }
        }
        panic!("not found")
    }
}