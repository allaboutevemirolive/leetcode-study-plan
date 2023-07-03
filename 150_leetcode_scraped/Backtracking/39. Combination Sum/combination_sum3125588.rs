// https://leetcode.com/problems/combination-sum/solutions/3125588/iterative-dfs-backtracking-solution-in-rust/
enum Action { Explore(usize), AddCandidate(i32), Restore }
impl Solution {
    pub fn combination_sum(candidates: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
        let mut ans = Vec::new();
        let (mut sum, mut combination) = (0, Vec::new());
        let mut stack = vec![Action::Explore(0)];
        while let Some(action) = stack.pop() {
            match action {
                Action::Explore(c_idx) => {
                    if sum < target && c_idx < candidates.len() {
                        let candidate = candidates[c_idx];
                        stack.push(Action::Restore);
                        stack.push(Action::Explore(c_idx));
                        stack.push(Action::AddCandidate(candidate));
                        stack.push(Action::Explore(c_idx+1));
                    }
                },
                Action::AddCandidate(candidate) => {
                    combination.push(candidate);
                    sum += candidate;
                    if sum==target {
                        ans.push(combination.clone());
                    }
                },
                Action::Restore => {
                    let candidate = combination.pop().unwrap();
                    sum -= candidate;
                },
            }
        }

        ans
    }
}