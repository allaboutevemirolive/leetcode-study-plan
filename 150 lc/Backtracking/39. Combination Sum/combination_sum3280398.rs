// https://leetcode.com/problems/combination-sum/solutions/3280398/rust-beats-100-memory-and-100-complexety-easy-recursive-sort-solution/
impl Solution {
    pub fn combination_sum(mut candidates: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
        let mut res = vec![];
        candidates.sort_unstable();
        Self::find(&candidates, target, &mut res, vec![]);
        res
    }
    pub fn find(candidates: &[i32], target: i32, res: &mut Vec<Vec<i32>>, cand: Vec<i32>) {
        if target == 0 {
            return res.push(cand)
        }
        candidates.iter().enumerate().take_while(|(_,&val)| val <= target).for_each(|(i,&val)| {
            let mut new = cand.clone();
            new.push(val);
            Self::find(&candidates[i..], target - val, res, new)
        })
    }
}