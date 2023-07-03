// https://leetcode.com/problems/symmetric-tree/solutions/3290374/rust-iterative-elixir-recursive/
use std::rc::Rc;
use std::cell::RefCell;
type OptNode = Option<Rc<RefCell<TreeNode>>>;

impl Solution {
    pub fn is_symmetric(root: OptNode) -> bool {
        if root.is_none() {
            return true;
        }
        let b = root.as_ref().unwrap().borrow();
        let mut v = vec![(b.left.clone(), b.right.clone())];
        while let Some(tuple) = v.pop() {
            match tuple {
                (None, None) => (),
                (Some(n1), Some(n2)) => {
                    let b1 = n1.borrow();
                    let b2 = n2.borrow();
                    if b1.val != b2.val {
                        return false;
                    }
                    v.push((b1.left.clone(), b2.right.clone()));
                    v.push((b1.right.clone(), b2.left.clone()));
                }
                _ => return false,
            }
        }
        true
    }
}