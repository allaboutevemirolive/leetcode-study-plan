// https://leetcode.com/problems/sum-root-to-leaf-numbers/solutions/3294431/concise-recursive-dfs-solution-in-rust/
use std::rc::Rc;
use std::cell::RefCell;
impl Solution {
    pub fn sum_numbers(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        fn dfs(node: &Option<Rc<RefCell<TreeNode>>>, path_sum: i32, sum: &mut i32) -> bool {
            node.as_ref().map(|node| {
                let TreeNode { val, ref left, ref right } = &*node.borrow();
                let path_sum = path_sum*10 + val;
                if dfs(left, path_sum, sum) & dfs(right, path_sum, sum) {
                    *sum += path_sum;
                }
            }).is_none()
        }
        let mut sum = 0;
        dfs(&root, 0, &mut sum);
        sum
    }
}