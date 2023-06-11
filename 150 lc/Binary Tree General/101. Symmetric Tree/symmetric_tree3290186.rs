// https://leetcode.com/problems/symmetric-tree/solutions/3290186/rust-concise-non-recursive-0ms/
use std::rc::Rc;
use std::cell::RefCell;

type NodeOpt = Option<Rc<RefCell<TreeNode>>>;

impl Solution {
    pub fn is_symmetric(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        let mut stack = {
            let root = root.as_ref().unwrap().borrow();
            vec![(root.left.clone(), root.right.clone())] 
        };
        while let Some(opts) = stack.pop() {
            match opts {
                (Some(p), Some(q)) => {
                    let (p, q) = (p.borrow(), q.borrow());
                    if p.val == q.val {
                        stack.push((p.right.clone(), q.left.clone()));
                        stack.push((p.left.clone(), q.right.clone()));
                    } else {
                        return false;
                    }
                },
                (None, None) => (),
                _ => return false,
            }
        }
        true
    }
}