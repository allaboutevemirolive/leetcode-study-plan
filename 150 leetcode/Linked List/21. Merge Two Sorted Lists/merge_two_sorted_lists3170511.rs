// https://leetcode.com/problems/merge-two-sorted-lists/solutions/3170511/rust-solution/

impl Solution {
    pub fn merge_two_lists(mut l1: Option<Box<ListNode>>,mut l2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
            let mut r = &mut l1;
        while l2.is_some() {
            if r.is_none() || l2.as_ref()?.val < r.as_ref()?.val {
                std::mem::swap(r, &mut l2);
            }
            r = &mut r.as_mut()?.next;
        }
        l1

    }
}