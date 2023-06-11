// https://leetcode.com/problems/construct-binary-tree-from-inorder-and-postorder-traversal/solutions/2956299/rust-dfs-easy-to-understand/
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
    pub fn build_tree(inorder: Vec<i32>, postorder: Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {
        
        fn dfs(inorder: &[i32], postorder: &[i32]) -> Option<Rc<RefCell<TreeNode>>> {
            match postorder.last() {
                Some(root) => {
                    let root_index = inorder.iter().enumerate().find(|(_, val)| &root == val).unwrap().0;
                    return Some(Rc::new(RefCell::new(TreeNode {
                        val: *root,
                        left: dfs(&inorder[0..root_index], &postorder[0..root_index]),
                        right: dfs(&inorder[(root_index+1)..], &postorder[root_index..(postorder.len()-1)])
                    })));
                },
                None => None
            }
        }
        dfs(&inorder, &postorder)
    }
}