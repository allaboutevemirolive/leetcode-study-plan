// https://leetcode.com/problems/binary-tree-zigzag-level-order-traversal/solutions/3204330/rust-simplest-solution-easy/
use std::rc::Rc;
use std::cell::RefCell;
use std::collections::VecDeque;

impl Solution {
    pub fn zigzag_level_order(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<Vec<i32>> {
        let mut ans : Vec<Vec<i32>> = vec![];
        let mut i = 0;
        let mut queue : VecDeque<Rc<RefCell<TreeNode>>> = VecDeque::new();

        if let Some(v) = root {
            queue.push_back(v);
        }
        
        while queue.len() > 0 {
            let lt = queue.len();
            let mut res : Vec<i32> = vec![];

            for i in 0..lt {
                let a = queue.pop_front();
                
                if let Some(v) = a{
                    let n = v.borrow();
                    res.push(n.val);
                    if let Some(left) = &n.left {
                        queue.push_back(Rc::clone(left));
                    }
                    
                    if let Some(right) = &n.right {
                        queue.push_back(Rc::clone(right));
                    }
                    
                }
            }   

            if (i&1) == 1 {
                res.reverse();
            }

            if res.len() > 0 {
                ans.push(res);
            }
            i += 1;
        }
        ans
    }
}