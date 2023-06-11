// https://leetcode.com/problems/merge-two-sorted-lists/solutions/2873267/rust-a-clean-fast-and-recursive-solution/
impl Solution {
    pub fn merge_two_lists(list1: Option<Box<ListNode>>, list2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        match (list1, list2) {
            (Some(node_a), Some(node_b)) => {
                let (mut smaller, bigger) = Self::sort(node_a, node_b);
                smaller.next = Self::merge_two_lists(smaller.next, Some(bigger));
                Some(smaller)
            }
            (node, None) | (None, node) => node,
            // The above will also match on `(None, None)` and return `None`, as it should.
        }
    }

    // Just a convenience function. It can be inlined, if one so prefers.
    // Pay no attention to the `const` marker. It's only there because it can be
    // and has no bearing on the problem at hand.
    const fn sort(list1: Box<ListNode>, list2: Box<ListNode>) -> (Box<ListNode>, Box<ListNode>) {
        if list1.val < list2.val {
            (list1, list2)
        } else {
            (list2, list1)
        }
    }
}