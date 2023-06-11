// https://leetcode.com/problems/binary-tree-level-order-traversal/solutions/2823245/rust-dfs-and-bfs-solution/
pub fn level_order(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<Vec<i32>> {
    let mut res = vec![];

    fn traverse_recursive(
        root: Option<Rc<RefCell<TreeNode>>>,
        res: &mut Vec<Vec<i32>>,
        lvl: usize,
    ) {
        if let Some(node) = root {
            if res.is_empty() || res.len() - 1 < lvl {
                res.push(vec![]);
            }
            let vv = &mut res[lvl];
            let node = node.borrow();
            vv.push(node.val);
            traverse_recursive(node.left.clone(), res, lvl + 1);
            traverse_recursive(node.right.clone(), res, lvl + 1);
        }
    }

    fn traverse_bfs(root: Option<Rc<RefCell<TreeNode>>>, res: &mut Vec<Vec<i32>>) {
        use std::collections::VecDeque;
        let mut q = VecDeque::new();
        if let Some(node) = root {
            q.push_back(node);
            while !q.is_empty() {
                let mut vv = vec![];
                for _ in 0..q.len() {
                    let node = q.pop_front().unwrap();
                    let node = node.borrow();
                    vv.push(node.val);
                    if let Some(node) = node.left.clone() {
                        q.push_back(node);
                    }
                    if let Some(node) = node.right.clone() {
                        q.push_back(node);
                    }
                }
                res.push(vv);
            }
        }
    }

    traverse_bfs(root, &mut res);
    res
}