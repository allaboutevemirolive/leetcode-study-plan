// https://leetcode.com/problems/merge-two-sorted-lists/solutions/3331122/zero-allocation-using-swap-or-take/
impl Solution {
    pub fn merge_two_lists(
        mut list1: Option<Box<ListNode>>,
        mut list2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        let mut dummy = None;
        let mut cur = &mut dummy;
        while list1.is_some() && list2.is_some() {
            if list1.as_ref().unwrap().val <= list2.as_ref().unwrap().val {
                // move list1 to cur
                std::mem::swap(cur, &mut list1);
                // move cur.next to list1
                std::mem::swap(&mut cur.as_mut().unwrap().next, &mut list1);
            } else {
                std::mem::swap(cur, &mut list2);
                std::mem::swap(&mut cur.as_mut().unwrap().next, &mut list2);
            }
            // update cur
            cur = &mut cur.as_mut().unwrap().next;
        }

        if list1.is_some() {
            std::mem::swap(cur, &mut list1);
        }
        if list2.is_some() {
            std::mem::swap(cur, &mut list2);
        }

        dummy
    }
}