// https://leetcode.com/problems/merge-two-sorted-lists/solutions/3218811/0-ms-rust-solution/
// Definition for singly-linked list.
// #[derive(PartialEq, Eq, Clone, Debug)]
// pub struct ListNode {
//   pub val: i32,
//   pub next: Option<Box<ListNode>>
// }
// 
// impl ListNode {
//   #[inline]
//   fn new(val: i32) -> Self {
//     ListNode {
//       next: None,
//       val
//     }
//   }
// }
impl Solution {
    pub fn merge_two_lists(l1: Option<Box<ListNode>>, l2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut dummy = ListNode::new(0);
        let mut tail = &mut dummy;
        let mut l1 = l1;
        let mut l2 = l2;

        while l1.is_some() && l2.is_some() {
            let (val, node) = if l1.as_ref().unwrap().val < l2.as_ref().unwrap().val {
                let mut node = l1.take().unwrap();
                l1 = node.next.take();
                (node.val, Some(node))
            } else {
                let mut node = l2.take().unwrap();
                l2 = node.next.take();
                (node.val, Some(node))
            };

            tail.next = node;
            tail = tail.next.as_mut().unwrap();
        }

        tail.next = if l1.is_some() { l1 } else { l2 };

        dummy.next
    }
}