// https://leetcode.com/problems/permutations/solutions/3122443/permutations-using-rust/
impl Solution {
    pub fn permute(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut res = vec![vec![]];
        for &i in nums.iter() {
            Self::permute_add(&mut res, i);
        }
        res.sort();
        res
    }

    pub fn permute_add(permutation: &mut Vec<Vec<i32>>, elem: i32) {
        if permutation.len() == 0 {
            permutation.push(vec![elem]);
            return
        }
        for _ in 0..permutation.len() {
            let base = permutation.remove(0);
            for i in 0..base.len() + 1 {
                let mut new_elem = base.clone();
                new_elem.insert(i, elem);
                permutation.push(new_elem);
            }
        }
    }
}