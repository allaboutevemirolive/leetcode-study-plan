// https://leetcode.com/problems/kth-smallest-element-in-a-bst/solutions/3197393/rust-0ms-solution-o-1-memory/
// Definition for a binary tree node.
// #[derive(Debug, PartialEq, Eq)]
// pub struct TreeNode {
//   pub val: i32,
//   pub left: Option<Rc<RefCell<TreeNode>>>,
//   pub right: Option<Rc<RefCell<TreeNode>>>,
// }
// 
// impl TreeNode {
//   #[inline]
//   pub fn new(val: i32) -> Self {
//     TreeNode {
//       val,
//       left: None,
//       right: None
//     }
//   }
// }
use std::rc::Rc;
use std::cell::RefCell;
impl Solution {
    pub fn kth_smallest(root: Option<Rc<RefCell<TreeNode>>>, mut k: i32) -> i32 {
        fn inorder(root: Option<Rc<RefCell<TreeNode>>>, k: &mut i32, ans: &mut i32) {
            if let Some(r) = root {
                let r = r.borrow();
                inorder(r.left.clone(), k, ans);
                *k -= 1;
                if *k == 0 {
                    *ans = r.val;
                }
                inorder(r.right.clone(), k, ans);
            }
        }
        let mut ans = 0;
        inorder(root, &mut k, &mut ans);
        ans
    }
}