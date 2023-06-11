// https://leetcode.com/problems/merge-two-sorted-lists/solutions/3514707/rust-recursion-in-place-0ms-beats-100/
type T = Option<Box<ListNode>>;
impl Solution {
    pub fn merge_two_lists(list1: T, list2: T) -> T {
        match (list1, list2) {
            (None, None) => None,
            (Some(n), None) | (None, Some(n)) => Some(n),
            (Some(list1), Some(list2)) => {
                if list1.val < list2.val {
                    Some(Box::new(ListNode {
                        val: list1.val,
                        next: Self::merge_two_lists(list1.next, Some(list2))
                    }))
                } else {
                    Some(Box::new(ListNode {
                        val: list2.val,
                        next: Self::merge_two_lists(Some(list1), list2.next)
                    }))
                }
            },
        }
    }
}