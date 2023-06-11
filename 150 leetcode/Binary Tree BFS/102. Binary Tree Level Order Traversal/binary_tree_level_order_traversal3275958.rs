// https://leetcode.com/problems/binary-tree-level-order-traversal/solutions/3275958/rust-solution-using-bfs/
use std::rc::Rc;
use std::cell::RefCell;
impl Solution {
  pub fn level_order(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<Vec<i32>> {
    let mut result = vec![];
    if let Some(node) = root {
      let mut stack = vec![node];
      while !stack.is_empty() {
        let mut temp = vec![];
        let mut new_stack = vec![];
        while let Some(node) = stack.pop() {
          temp.push(node.borrow().val);
          if let Some(left) = &node.borrow().left {
            new_stack.push(left.clone());
          }
          if let Some(right) = &node.borrow().right {
            new_stack.push(right.clone());
          }
        }

        if !temp.is_empty() {
          result.push(temp);
        }
        new_stack.reverse();
        stack = new_stack;
      }
      result
    }  else {
      result
    }     
  }
}