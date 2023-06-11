// https://leetcode.com/problems/binary-tree-zigzag-level-order-traversal/solutions/3207552/rust-bfs-but-stack-instead-of-queue/
use std::rc::Rc;
use std::cell::RefCell;
impl Solution {
    pub fn zigzag_level_order(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<Vec<i32>> {
        let mut is_ltr = true;
        let mut ans = vec![];
        let mut lvl = if root.is_some() { vec![root] } else { vec![] };
        let mut next_lvl = vec![];
        while !lvl.is_empty() {
            let mut values = vec![];
            while let Some(node) = lvl.pop() {
                if let Some(node) = node {
                    let TreeNode { ref left, ref right, val } = &*node.borrow();
                    let children = if is_ltr { [left, right] } else { [right, left] };
                    next_lvl.extend(children.into_iter().filter_map(|opt_node| opt_node.as_ref().map(|node| Some(node.clone()) )));
                    values.push(*val);
                }
            }
            ans.push(values);
            std::mem::swap(&mut lvl, &mut next_lvl);
            is_ltr ^= true;
        }
        ans
    }
}