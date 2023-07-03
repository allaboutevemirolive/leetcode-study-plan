// https://leetcode.com/problems/invert-binary-tree/solutions/1789032/rust-bfs-dfs-iterative/
type TreeLink = Option<Rc<RefCell<TreeNode>>>;

impl Solution {
    fn invert_tree_bfs(root: TreeLink) -> TreeLink {
        let mut queue: VecDeque<TreeLink> = VecDeque::new();
        queue.push_front(root.clone());
        while let Some(ele) = queue.pop_back() {
            if let Some(n) = ele {
				// method 1.
                let TreeNode { left, right, .. } = &mut *n.borrow_mut();
                mem::swap(right, left);
				queue.push_back(left.clone());
                queue.push_back(right.clone());

				// method 2.
				let mut borrowed_node = n.borrow_mut();
				let borrowed_node = &mut *borrowed_node;
				mem::swap(&mut borrowed_node.right, &mut borrowed_node.left);
				queue.push_back(borrowed_node.left.clone());
                queue.push_back(borrowed_node.right.clone());
            }
        }
        root
    }
}