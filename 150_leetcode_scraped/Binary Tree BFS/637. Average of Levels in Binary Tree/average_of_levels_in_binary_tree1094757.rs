// https://leetcode.com/problems/average-of-levels-in-binary-tree/solutions/1094757/rust-bfs/
impl Solution {
    pub fn average_of_levels(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<f64> {
        let mut res = vec![];
        // BFS -> calc avg on each level, push to res.
        let mut queue = VecDeque::new();
        if let Some(root) = root {
            queue.push_back(root);
        }
        while queue.len() > 0 {
            let len = queue.len();
            let mut acc = 0.0;
            for _ in 0..len {
                let node = queue.pop_front().unwrap();
                acc += node.borrow().val as f64;
                if let Some(ref l) = node.clone().borrow().left {
                    queue.push_back(l.clone());
                }
                if let Some(ref r) = node.clone().borrow().right {
                    queue.push_back(r.clone());
                }
            }
            res.push(acc / len as f64);
        }
        res
    }
}