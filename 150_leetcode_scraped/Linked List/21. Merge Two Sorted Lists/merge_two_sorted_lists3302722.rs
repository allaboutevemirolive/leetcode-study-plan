// https://leetcode.com/problems/merge-two-sorted-lists/solutions/3302722/rust-recursive/
impl Solution {
    pub fn merge_two_lists(
        list1: Option<Box<ListNode>>,
        list2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        // invariant: always return the smaller value of list1 and list2
        match (list1, list2) {
            (None, None) => None,
            (None, Some(boxed_v)) | (Some(boxed_v), None) => Some(boxed_v),
            // use mut because we need to change .next
            (Some(mut boxed_v1), Some(mut boxed_v2)) => {
                if boxed_v1.val < boxed_v2.val {
                    boxed_v1.next = Solution::merge_two_lists(boxed_v1.next, Some(boxed_v2));
                    Some(boxed_v1)
                } else {
                    boxed_v2.next = Solution::merge_two_lists(Some(boxed_v1), boxed_v2.next);
                    Some(boxed_v2)
                }
            }
        }
    }
}