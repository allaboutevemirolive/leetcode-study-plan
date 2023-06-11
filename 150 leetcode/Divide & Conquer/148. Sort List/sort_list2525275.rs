// https://leetcode.com/problems/sort-list/solutions/2525275/rust-merge-sort/
impl Solution {
    pub fn sort_list(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {

        let mut head = head;

        if head.is_none() || head.as_ref().unwrap().next.is_none() {
            return head;
        }

        let tmp_head = head.clone();
        let mut fast = tmp_head.as_ref().unwrap().next.as_ref();
        let mut slow = head.as_mut();

        while fast.is_some() && fast.unwrap().next.is_some() {
            slow = slow.unwrap().next.as_mut();
            fast = fast.unwrap().next.as_ref().unwrap().next.as_ref();
        }

        let mid = slow.unwrap().next.take();

        let left = Self::sort_list(head);
        let right = Self::sort_list(mid);
        Self::merge(left, right)

    }

    fn merge(mut left: Option<Box<ListNode>>, mut right: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        match(left, right) {
            (None, None) => None,
            (Some(node), None) => Some(node),
            (None, Some(node)) => Some(node),
            (Some(l_node), Some(r_node)) => {
                if l_node.val < r_node.val {
                    Some(Box::new(ListNode { val: l_node.val, next: Self::merge(l_node.next, Some(r_node)) }))
                } else {
                    Some(Box::new(ListNode { val: r_node.val, next: Self::merge(Some(l_node), r_node.next) }))
                }
            }
        }
    }
}