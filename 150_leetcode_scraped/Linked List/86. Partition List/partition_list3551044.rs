// https://leetcode.com/problems/partition-list/solutions/3551044/rust-0ms-faster-than-100-submit/
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
    pub fn partition(head: Option<Box<ListNode>>, x: i32) -> Option<Box<ListNode>> {
        if head.is_none() || head.as_ref().unwrap().next.is_none() {
            return head;
        }

        let mut large = Box::new(ListNode::new(999));
        let mut car_lar = &mut large.next;
        let mut lower = Box::new(ListNode::new(0));
        let mut car_lower = &mut lower.next;

        let mut dummy = head;
        while let Some(node) = dummy {
            if node.val >= x {
                *car_lar = Some(Box::new(ListNode::new(node.val)));
                car_lar = &mut car_lar.as_mut().unwrap().next;
            } else {
                *car_lower = Some(Box::new(ListNode::new(node.val)));
                car_lower = &mut car_lower.as_mut().unwrap().next;
            }

            dummy = node.next;
        }

        *car_lower = large.next;
        
        lower.next
    }
}