// https://leetcode.com/problems/construct-binary-tree-from-inorder-and-postorder-traversal/solutions/1537290/rust-solution-100-2-6mb-100/
use std::rc::Rc;
use std::cell::RefCell;
type Node = Option<Rc<RefCell<TreeNode>>>;
impl Solution {
    pub fn build_tree(inorder: Vec<i32>, postorder: Vec<i32>) -> Node {
        Solution::builder(&inorder[..], &postorder[..])
    }
    fn builder(inorder: &[i32], postorder: &[i32]) -> Node { 
        let i = inorder.len();
        if inorder.is_empty() || postorder.is_empty() { 
            return None 
        } else { 
            let n = postorder.len() -1;
            let mut root = TreeNode::new(postorder[n]);
            let m = inorder.iter().position(|&x| x == postorder[n]).unwrap();
            root.left = Solution::builder(&inorder[0..m], &postorder[0..m]);
            root.right = Solution::builder(&inorder[m+1..i], &postorder[m..i-1]);
            Some(Rc::new(RefCell::new(root)))
        }
    }
}