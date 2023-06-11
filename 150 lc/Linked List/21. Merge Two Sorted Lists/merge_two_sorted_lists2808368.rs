// https://leetcode.com/problems/merge-two-sorted-lists/solutions/2808368/rust-version-of-leetcode-official-solution/
// Rust version of LeetCode official solution.
impl Solution {
    pub fn merge_two_lists(
        list1: Option<Box<ListNode>>,
        list2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        if list1.is_none() {
            return list2;
        }
        if list2.is_none() {
            return list1;
        }
        if let Some(mut node1) = list1 {
            if let Some(mut node2) = list2 {
                if node1.val < node2.val {
                    node1.next = Self::merge_two_lists(node1.next, Some(node2));
                    return Some(Box::new(*node1));
                } else {
                    node2.next = Self::merge_two_lists(Some(node1), node2.next);
                    return Some(Box::new(*node2));
                }
            }
        }
        None
    }
}