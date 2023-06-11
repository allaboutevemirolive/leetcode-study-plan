// https://leetcode.com/problems/average-of-levels-in-binary-tree/solutions/1099883/rust-solution-using-bfs/
    let mut res: Vec<f64> = Vec::new();
    let mut queue: Vec<Rc<RefCell<TreeNode>>> = Vec::new();
    if let Some(r) = root {
        queue.push(r);
    }
    while(!queue.is_empty()) {
        let mut count = queue.len();
        let mut sum:f64 = 0.0;
        let mut tmp: Rc<RefCell<TreeNode>> = 
            Rc::new(RefCell::new(TreeNode::new(0)));
        for i in 0..count {
            tmp = queue[i].clone();
            sum += tmp.borrow().val as f64;
            if let Some(l) = &tmp.borrow().left {
                queue.push(l.clone());
            }
            if let Some(r) = &tmp.borrow().right {
                queue.push(r.clone());
            }
        }
        queue.drain(0..count);
        res.push(sum / count as f64);
    }
    res
}