// https://leetcode.com/problems/merge-two-sorted-lists/solutions/2866594/rust-0ms/
impl Solution {
    pub fn merge_two_lists(
        list1: Option<Box<ListNode>>,
        list2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        match (list1, list2) {
            (Some(l1), None) => Some(l1),
            (None, Some(l2)) => Some(l2),
            (None, None)     => None,
            (Some(l1), Some(l2)) => match l1.val <= l2.val {
                true  => Some(Box::new(ListNode {
                    val: l1.val,
                    next: Self::merge_two_lists(l1.next, Some(l2)),
                })),
                false => Some(Box::new(ListNode {
                    val: l2.val,
                    next: Self::merge_two_lists(Some(l1), l2.next),
                })),
            },
        }
    }
}