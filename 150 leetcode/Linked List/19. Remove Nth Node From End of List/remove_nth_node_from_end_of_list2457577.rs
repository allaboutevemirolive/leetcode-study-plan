// https://leetcode.com/problems/remove-nth-node-from-end-of-list/solutions/2457577/rust-solution-recursion-0ms-2mb/
impl Solution {
    pub fn remove_nth_from_end(head: Option<Box<ListNode>>, n: i32) -> Option<Box<ListNode>> {
        Self::build_list(head, n).1
    }

    fn build_list(node: Option<Box<ListNode>>, n: i32) -> (i32, Option<Box<ListNode>>) {
        if let Some(mut _node) = node {
            let (mut i, tmp_node) = Self::build_list(_node.next, n);
            i += 1;
            if i == n {
                (i, tmp_node)
            } else {
                _node.next = tmp_node;
                (i, Some(_node))
            }
        } else {
            (0, None)
        }
    }
}