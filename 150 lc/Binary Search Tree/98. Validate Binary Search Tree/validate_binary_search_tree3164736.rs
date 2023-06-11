// https://leetcode.com/problems/validate-binary-search-tree/solutions/3164736/rust-iterative-and-recursive-traversal-with-ranges/
use std::rc::Rc;
use std::cell::RefCell;

type Node = Option<Rc<RefCell<TreeNode>>>;


impl Solution {
    pub fn is_valid_bst(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        fn _is_valid_bst(root: Node) -> bool {
            let mut stack : Vec<(Node, i64, i64)> = vec![(root, i64::MIN, i64::MAX)];

            while let Some(ele) = stack.pop() {
                if let (Some(node), left, right) = ele {
                    let node = node.borrow();
                    let val = node.val as i64;
                    if left >= val || val >= right {
                        return false;
                    }
                    stack.push((node.right.clone(), val, right));
                    stack.push((node.left.clone(), left, val));
                }
            }
            true
        }


        _is_valid_bst(root)
        
    }
}