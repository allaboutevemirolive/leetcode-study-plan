// https://leetcode.com/problems/same-tree/solutions/1680419/rust/
    pub fn is_same_tree(p: Option<Rc<RefCell<TreeNode>>>, q: Option<Rc<RefCell<TreeNode>>>) -> bool {
      fn helper(p: &Option<Rc<RefCell<TreeNode>>>, q: &Option<Rc<RefCell<TreeNode>>>) -> bool {
        match (p, q) {
          (None, None) => true,
          (Some(np), Some(nq)) => {
            let np = np.borrow();
            let nq = nq.borrow();
            np.val == nq.val && helper(&np.left, &nq.left) && helper(&np.right, &nq.right)
          },
          _ => false
        }
      }

      helper(&p, &q)
    }