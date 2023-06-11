// https://leetcode.com/problems/same-tree/solutions/1260133/rust-iterative-solution/
use std::rc::Rc;
use std::cell::RefCell;

impl Solution 
{
    pub fn is_same_tree(p: Option<Rc<RefCell<TreeNode>>>, q: Option<Rc<RefCell<TreeNode>>>) -> bool 
    {
        let mut v = vec![(p, q)];
        
        while !v.is_empty()
        {
            if !match (v.pop().unwrap()) { 
                (None, None) => true,
                
                (Some(p), Some(q)) => {
                    let p = p.borrow();
                    let q = q.borrow();
                    
                    if p.val != q.val {
                        false
                    } else {
                        v.push( (p.left.clone(), q.left.clone()) );
                        v.push( (p.right.clone(), q.right.clone()) );
                        true
                    }
                },
                
                _ => false
                
            } { return false; }
        }
        
        true
        
    }
}