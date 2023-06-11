// https://leetcode.com/problems/remove-nth-node-from-end-of-list/solutions/3384605/rust-2-pass-o-n-runtime-o-1-space/
type Node = Box<ListNode>;

impl Solution {
    fn remove_node(node: &mut Option<Node>, idx: i32, idx_to_remove: i32) {
        if let Some(n) = node {
            if idx_to_remove == 0 {
                *node = n.next.take();
            } else if idx == idx_to_remove - 1 {
                if let Some(next) = n.next.as_mut() {
                    n.next = next.next.take();
                } else {
                    node.take();
                }
            } else {
                Self::remove_node(&mut n.next, idx + 1, idx_to_remove);
            }
        }
    }

    fn find_last_node(node: Option<&Node>, last_node_idx: &mut i32) {
        if let Some(node) = node {
            Self::find_last_node(node.next.as_ref(), last_node_idx);
            *last_node_idx += 1;
        }
    }

    pub fn remove_nth_from_end(mut head: Option<Node>, n: i32) -> Option<Node> {
        let mut last_node_idx = 0;
        Self::find_last_node(head.as_ref(), &mut last_node_idx);
        let node_to_remove = last_node_idx - n;
        Self::remove_node(&mut head, 0, node_to_remove);
        head
    }
}
