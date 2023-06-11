// https://leetcode.com/problems/average-of-levels-in-binary-tree/solutions/1859155/rust-0ms-2-9mb/
use std::rc::Rc;
use std::cell::RefCell;
use std::collections::VecDeque;
impl Solution {
    pub fn average_of_levels(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<f64> {
        let (mut res, mut q) = (vec![], VecDeque::new());
        q.push_back((root, 0));
        
        while let Some((node, lvl)) = q.pop_front() { 
            if let Some(node) = node { 
                let node = node.borrow();
                if res.len() <= lvl { 
                    res.resize(lvl + 1, vec![]);                
                }
                res[lvl].push(node.val as i64);
                q.push_back((node.left.clone(), lvl + 1));
                q.push_back((node.right.clone(), lvl + 1));
            }
        }
        res.into_iter()
        .map(|top| top.iter().sum::<i64>() as f64/ top.len() as f64)
        .collect()
        
        
    }
}