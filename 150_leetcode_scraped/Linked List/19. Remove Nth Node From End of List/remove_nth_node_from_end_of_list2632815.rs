// https://leetcode.com/problems/remove-nth-node-from-end-of-list/solutions/2632815/rust-yars-yet-another-recursive-solution/
type OptNode = Option<Box<ListNode>>;

impl Solution {
    pub fn remove_nth_from_end2(head: OptNode, n: i32) -> OptNode {
        Self::remove_util(head, n).0
    }

    fn remove_util(mut node: OptNode, n: i32) -> (OptNode, i32) {
        match node.as_mut() {
            None => (None, 0),
            Some(node_box) => {
                let (node2, i) = Self::remove_util(node_box.next.take(), n);
                if i + 1 == n {
                    (node2, i + 1)
                }
                else {
                    node_box.next = node2;
                    (node, i + 1)
                }
            }
        }
    }
}