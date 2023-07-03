// https://leetcode.com/problems/convert-sorted-array-to-binary-search-tree/solutions/3311185/rust-simple-recursive-solution/
use std::rc::Rc;
use std::cell::RefCell;

impl Solution {
    pub fn sorted_array_to_bst(nums: Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {
      return Solution::helper(&nums,0,nums.len() as i32);
    }
    fn helper(nums:&Vec<i32>,i:i32,j:i32)->Option<Rc<RefCell<TreeNode>>>{
      if i>j || j<0 || i>=nums.len() as i32 {
        return None;
      }
      let mid = (i+(j-i)/2) as usize;
      Some(Rc::new(RefCell::new(TreeNode{
        val: nums[mid],
        left: Solution::helper(nums,i,mid as i32-1),
        right: Solution::helper(nums,mid as i32 +1,j),
      })))
    }
}